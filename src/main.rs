/*
 * Rustの型。
 * CreatedAt: 2019-05-30
 */
fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
//  let guess      = "42".parse().expect("Not a number!"); // error[E0282]: type annotations needed
    println!("i8   {}..{}", i8::min_value(), i8::max_value());
    println!("i16  {}..{}", i16::min_value(), i16::max_value());
    println!("i32  {}..{}", i32::min_value(), i32::max_value());
    println!("i64  {}..{}", i64::min_value(), i64::max_value());
    println!("i128 {}..{}", i128::min_value(), i128::max_value());
    println!("u8   {}..{}", u8::min_value(), u8::max_value());
    println!("u16  {}..{}", u16::min_value(), u16::max_value());
    println!("u32  {}..{}", u32::min_value(), u32::max_value());
    println!("u64  {}..{}", u64::min_value(), u64::max_value());
    println!("u128 {}..{}", u128::min_value(), u128::max_value());

//    let i: i32 = String.From("123").parse().unwrap().expect("Not a number.");
    let i: i32 = "123".parse().expect("Not a number.");
    println!("let i: i32 = \"123\".parse().expect(\"Not a number.\");  {}", i);
    println!("i32.to_string() {}", i.to_string());
    println!("format!() {}", format!("{}", i));

    let f: f32 = 3.14;
    println!("f32  {} floor={}", f, f.floor());
    let f: f64 = 3.14;
    println!("f64  {} floor={}", f, f.floor());

    // メモリ内の任意位置を示すのに必要なサイズ。32bitマシンなら32bit。
    println!("isize {}..{}", isize::min_value(), isize::max_value());
    println!("usize {}..{}", usize::min_value(), usize::max_value());
}

