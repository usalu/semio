//! 🔢️ Drives the native scalar API over the neutral integer workload.

use semio_plugin_interpreter_numeric::{execute_numeric_fixed, Value};
use std::time::Instant;

fn opcodes(raw: &str) -> Vec<u8> {
    raw.split(',').map(|opcode| opcode.parse().unwrap()).collect()
}

fn main() {
    let mut arguments = std::env::args().skip(1);
    let iterations: i32 = arguments.next().unwrap().parse().unwrap();
    let seed: i32 = arguments.next().unwrap().parse().unwrap();
    let i32_ops = opcodes(&arguments.next().unwrap());
    let i64_ops = opcodes(&arguments.next().unwrap());
    let mut narrow = Value::I32(seed);
    let mut wide = Value::I64(i64::from(seed));
    let started = Instant::now();
    for index in 0..iterations {
        let parameter = seed.wrapping_add(index).wrapping_mul(3);
        for &opcode in &i32_ops {
            narrow = execute_numeric_fixed(opcode, &[narrow, Value::I32(parameter)]).unwrap();
        }
        let parameter = i64::from(parameter).wrapping_mul(3_754_819);
        for &opcode in &i64_ops {
            wide = execute_numeric_fixed(opcode, &[wide, Value::I64(parameter)]).unwrap();
        }
    }
    let elapsed = started.elapsed().as_nanos();
    let (Value::I32(narrow), Value::I64(wide)) = (narrow, wide) else { unreachable!() };
    println!(r#"{{"nanoseconds":{elapsed},"i32":{narrow},"i64":"{wide}"}}"#);
}
