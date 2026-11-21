//! 🧮 Verifies the extracted actual scalar owner against independent WASM cases.
use semio_plugin_interpreter_scalar::{Value, NumericError, execute_numeric_fixed};
#[test]
fn exact_scalar_semantics() {
    assert_eq!(execute_numeric_fixed(109, &[Value::I32(-21), Value::I32(4)]), Ok(Value::I32(-5)), "i32 signed divide");
    assert_eq!(execute_numeric_fixed(109, &[Value::I32(33), Value::I32(0)]), Err(NumericError::Division), "i32 division by zero");
    assert_eq!(execute_numeric_fixed(109, &[Value::I32(-2147483648), Value::I32(-1)]), Err(NumericError::Overflow), "i32 signed overflow");
    assert_eq!(execute_numeric_fixed(110, &[Value::I32(-1), Value::I32(2)]), Ok(Value::I32(2147483647)), "i32 unsigned divide");
    assert_eq!(execute_numeric_fixed(106, &[Value::I32(2147483647), Value::I32(1)]), Ok(Value::I32(-2147483648)), "i32 wrap");
    assert_eq!(execute_numeric_fixed(158, &[Value::f64(2.5)]), Ok(Value::f64(2.0)), "f64 ties-even");
    assert_eq!(execute_numeric_fixed(165, &[Value::f64(-0.0), Value::f64(0.0)]), Ok(Value::f64(0.0)), "f64 max signed zero");
    assert_eq!(execute_numeric_fixed(164, &[Value::f64(0.0), Value::f64(-0.0)]), Ok(Value::f64(-0.0)), "f64 min signed zero");
    assert_eq!(execute_numeric_fixed(170, &[Value::f64(f64::NAN)]), Err(NumericError::Conversion), "float NaN truncation");
    assert_eq!(execute_numeric_fixed(188, &[Value::f32(1.5)]), Ok(Value::I32(1069547520)), "f32 naive reinterp");
}
