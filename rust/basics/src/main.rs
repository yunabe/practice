// Naing conventions
// https://doc.rust-lang.org/style/style/naming/README.html

use std::mem;

// 1. Variable Bindings
// https://doc.rust-lang.org/book/variable-bindings.html
fn play_with_variables() {
    println!("==== play_play_with_variables ====");
    {
        let x = 3;
        let y: i32 = 4;
        let (a, b) = (7, 8);
        let (c, d): (f32, f64) = (1.2, 3.4);

        // https://doc.rust-lang.org/beta/book/casting-between-types.html#as
        let z: i64 = x + y as i64;
        println!("z == {}", z);
        println!("a + b == {}", a + b);
        println!("c + d == {}", c as f64 + d);
    }
    {
        let x = 3;
        // Error
        // x += 10;

        let mut y = 4;
        y += x;
        println!("y == {}", y);
    }
    {
        let x;
        // Error: Rust does not zero-initialze variables.
        // println!("x == {}", x);
        x = 10;
        println!("x == {}", x);

        // error: re-assignment of immutable variable `x`
        // x = 20;
    }
    {
        let x = 20;
        println!("x == {}", x);
        // Shadowing
        let x = 3.4;
        println!("x == {}", x);

        let mut y = 0;
        y += 1;
        // Make y immutable.
        let y = y;

        // error: re-assignment of immutable variable `y`
        // y += 1;
        println!("y == {}", y);
    }
}

// 2. Functions
// https://doc.rust-lang.org/book/functions.html
fn naive_fib(n: i32) -> i32 {
    if n < 2 {
        return 1;
    }
    return naive_fib(n - 1) + naive_fib(n - 2);
}

fn return_multiple_values() -> (i32, i32) {
    let a = 3;
    return (a, 4);
}

// ! is diverges.
// https://doc.rust-lang.org/book/functions.html#diverging-functions
fn always_panic() -> ! {
    panic!("always_panic should not be called.");
}

fn receive_fn(f: fn(i32) -> i32) {
    let n = 5;
    println!("f({}) = {}", n, f(n));
}

fn play_with_functions(panic: bool) {
    println!("==== play_with_functions ====");
    {
        let n = 15;
        println!("naive_fib({}) = {}", n, naive_fib(n));
    }
    {
        let (a, b) = return_multiple_values();
        println!("a = {}, b = {}", a, b);
    }
    if panic {
        // diverges can be assigned to any type of variables.
        let (x, y): (i32, f32) = always_panic();
        println!("x, y = {}, {}", x, y);
    }
    receive_fn(naive_fib);
}

// 3. Primitive types
// https://doc.rust-lang.org/book/primitive-types.html
fn play_with_primitives() {
    println!("=== play_with_primitives ===");
    {
        // Boaleans
        let b: bool = true;  // or false.
        println!("b is {}", b);
    }
    {
        // char is a single Unicode scalar value.
        let c: char = 'x';
        let multi_c: char = '犬';
        let emoji: char = '😺';
        println!("c: {}, multi_c: {}, emoji: {}", c, multi_c, emoji);
    }
    {
        // Numeric types
        // https://doc.rust-lang.org/book/primitive-types.html#numeric-types

        // signed integers. a and b are overflowing (warning).
        let (a, b, c, d): (i8, i16, i32, i64) = (70000, 70000, 70000, 70000);
        println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);
        // unsigned integers.
        let (a, b, c, d): (u8, u16, u32, u64) = (70000, 70000, 70000, 70000);
        println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);

        // isize (signed) and usize (unsigned) depend on the underlying machine architecture.
        let (is, us): (isize, usize) = (2, 3);
        println!("is: {}, us: {}", is, us);
        println!("sizeof(is): {}, sizeof(us): {}",
                 mem::size_of_val(&is),
                 mem::size_of_val(&us));

        let (f0, f1): (f32, f64) = (1.2, 3.4);
        println!("f0: {}, f1: {}", f0, f1);
    }
    {
        // Arrays
        // https://doc.rust-lang.org/book/primitive-types.html#Arrays
        let a = [1, 2, 3];  // a: [i32; 3]
        // initialized array.
        let mut b = [0; 10];  // a: [i32: 10]
        b[1] = 1;
        println!("a: {:?}, b: {:?}", a, b);
        println!("b.len() == {}", b.len());

        // Slices
        {
            let s = &b[1..4];
            println!("s: {:?}, s.len(): {}", s, s.len());
        }
        {
            // the mutable slice type.
            // https://doc.rust-lang.org/std/primitive.slice.html
            // ms can not exist together with s above.
            let ms = &mut b[..];
            ms[0] = 9;
            println!("ms: {:?}, ms.len(): {}", ms, ms.len());
        }
        // `ms[0] = 9` affects to `b`.
        println!("b: {:?}", b);
    }
    {
        // Tuples
        // https://doc.rust-lang.org/book/primitive-types.html#tuples
        let t: (i32, &str) = (42, "Hello");
        println!("t: {:?}", t);
        let (x, y) = t;
        println!("x: {}, y: {}", x, y);
        println!("t.0: {}, t.1: {}", t.0, t.1);

        // single-element tuple
        let single = (8,);
        println!("single: {:?}", single);
    }
}

// 4. Comments
// https://doc.rust-lang.org/book/comments.html

// 5, 6. Flow controls
// if: https://doc.rust-lang.org/book/if.html
// Loops: https://doc.rust-lang.org/book/loops.html
fn play_with_flow_controls() {
    println!("=== play_with_flow_controls ===");
    let n = 10;
    if n % 5 == 0 && n % 3 == 0 {
        println!("Fizz Buzz");
    } else if n % 3 == 0 {
        println!("Fizz");
    } else if n % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{}", n);
    }

    let mut index = 0;
    // infinite-loop.
    loop {
        if index > 5 {
            break;
        }
        println!("in loop : index: {}", index);
        index += 1;
    }
    // while-loop.
    while index < 10 {
        println!("in while: index: {}", index);
        index += 1;
    }
    // for-loop.
    // C-style for is not supported in Rust.
    for index in 10..15 {
        // index = 10, 11, 12, 13, 14.
        // the new variable `index` is defined in this scope.
        println!("index: {}", index);
    }
    println!("in while: index: {}", index);

    // Labeled-break/continue
    'outer: for i in 0..10 {
        for j in 0..10 {
            if i * j > 15 {
                println!("i * j exceeded 15 ({}, {})", i, j);
                break 'outer;
            }
        }
    }
    {
        // In Rust, almost everything is expression.
        // `if` is expression too in Rust.
        let n = 10;
        let a = if n % 2 == 0 { "even" } else { "odd" };
        println!("n is {}", a);

        // if (w/o else), while, loop is evaluated as () (an empty tuple).
        // Personally, I do not like the Rust's philosophy to handle
        // everything as expression so much. It looks just error-prone for me.
        let ifonly_val = if true {
            println!("inside-if!");
        };
        let while_val = while n > 0 {
            println!("inside-while!");
            break;
        };
        let loop_val = loop {
            println!("inside-loop!");
            break;
        };
        println!("ifonly_val: {:?}, while_val: {:?}, loop_val: {:?}",
                 ifonly_val,
                 while_val,
                 loop_val);
    }
}

fn main() {
    play_with_variables();
    play_with_functions(false);
    play_with_primitives();
    play_with_flow_controls();
}
