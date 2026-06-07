//! ➕ Flow math module: neuron kinds for arithmetic.

use neural_engine::{Atom, Dictionary, EvalError, Function, Registry, Value};

// #region 🔖Add
/// ➕ Sums two number inputs into one number output.
pub struct Add;

impl Function for Add {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let a = read_number(input, "a").or_else(|_| read_number(input, "number"))?;
        let b = read_number(input, "b").unwrap_or(0.0);
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(a + b))))
    }
}
// #endregion 🔖Add

// #region 🔖Multiply
/// ✖️ Multiplies two number inputs.
pub struct Multiply;

impl Function for Multiply {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let a = read_number(input, "a")?;
        let b = read_number(input, "b")?;
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(a * b))))
    }
}
// #endregion 🔖Multiply

// #region 🔖PassThrough
/// ➡️ Forwards the number input unchanged.
pub struct PassThrough;

impl Function for PassThrough {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let n = read_number(input, "number")?;
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(n))))
    }
}
// #endregion 🔖PassThrough

fn read_number(input: &Dictionary, key: &str) -> Result<f64, EvalError> {
    input
        .get(key)
        .and_then(|v| v.as_atom())
        .and_then(|a| a.as_f64())
        .ok_or_else(|| EvalError::MissingInput(key.into()))
}

/// 📦 Registers all math neuron kinds on the registry.
pub fn register(registry: &mut Registry) {
    registry.register("math.add", Box::new(Add));
    registry.register("math.multiply", Box::new(Multiply));
    registry.register("math.passThrough", Box::new(PassThrough));
}

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_sums_inputs() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new().insert("a", Value::Atom(Atom::Decimal(3.0))).insert("b", Value::Atom(Atom::Decimal(1.1)));
        let out = reg.get("math.add").unwrap().evaluate(&input).unwrap();
        assert_eq!(out.get("number").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(4.1));
    }
}
// #endregion 🔖Tests
