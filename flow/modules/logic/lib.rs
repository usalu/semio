//! 🔀 Flow logic module: neuron kinds for boolean comparisons.

use neural_engine::{Atom, Dictionary, EvalError, Function, NeuronKindInfo, Registry, Value};

// #region 🔖Greater
/// 📈 Compares two numbers; outputs 1 when a > b else 0.
pub struct Greater;

impl Function for Greater {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let a = read_number(input, "a")?;
        let b = read_number(input, "b")?;
        let flag = if a > b { 1.0 } else { 0.0 };
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(flag))))
    }
}
// #endregion 🔖Greater

// #region 🔖Not
/// 🔄 Inverts a boolean number (1 -> 0, 0 -> 1).
pub struct Not;

impl Function for Not {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let n = read_number(input, "number")?;
        let flag = if n > 0.0 { 0.0 } else { 1.0 };
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(flag))))
    }
}
// #endregion 🔖Not

fn read_number(input: &Dictionary, key: &str) -> Result<f64, EvalError> {
    input
        .get(key)
        .and_then(|v| v.as_atom())
        .and_then(|a| a.as_f64())
        .ok_or_else(|| EvalError::MissingInput(key.into()))
}

/// 📦 Registers all logic neuron kinds on the registry.
pub fn register(registry: &mut Registry) {
    registry.register(
        NeuronKindInfo {
            id: "logic.greater".into(),
            module: "logic".into(),
            name: "Greater".into(),
            summary: "True when a > b".into(),
            inputs: vec!["a".into(), "b".into()],
            outputs: vec!["number".into()],
        },
        Box::new(Greater),
    );
    registry.register(
        NeuronKindInfo {
            id: "logic.not".into(),
            module: "logic".into(),
            name: "Not".into(),
            summary: "Inverts boolean number".into(),
            inputs: vec!["number".into()],
            outputs: vec!["number".into()],
        },
        Box::new(Not),
    );
}

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greater_compares_numbers() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new()
            .insert("a", Value::Atom(Atom::Decimal(5.0)))
            .insert("b", Value::Atom(Atom::Decimal(2.0)));
        let out = reg.get("logic.greater").unwrap().evaluate(&input).unwrap();
        assert_eq!(out.get("number").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(1.0));
    }
}
// #endregion 🔖Tests
