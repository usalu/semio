unsafe extern "C" { fn native_c_probe() -> i32; }
fn main() { assert_eq!(unsafe { native_c_probe() }, 1); println!("native C headers and linker work"); }
