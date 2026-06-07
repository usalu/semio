//! 📝 Flow text module: neuron kinds for strings.

use neural_engine::{Atom, Dictionary, EvalError, Function, NeuronKindInfo, Registry, Value};

// #region 🔖Concat
/// 🔗 Joins two text inputs.
pub struct Concat;

impl Function for Concat {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let a = read_text(input, "a").or_else(|_| read_text(input, "text"))?;
        let b = read_text(input, "b").unwrap_or_default();
        Ok(Dictionary::new().insert("text", Value::Atom(Atom::String(format!("{a}{b}")))))
    }
}
// #endregion 🔖Concat

// #region 🔖Upper
/// 🔠 Uppercases a text input.
pub struct Upper;

impl Function for Upper {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let text = read_text(input, "text")?;
        Ok(Dictionary::new().insert("text", Value::Atom(Atom::String(text.to_uppercase()))))
    }
}
// #endregion 🔖Upper

fn read_text(input: &Dictionary, key: &str) -> Result<String, EvalError> {
    input
        .get(key)
        .and_then(|v| v.as_atom())
        .and_then(|a| a.as_str())
        .map(str::to_string)
        .ok_or_else(|| EvalError::MissingInput(key.into()))
}

/// 📦 Registers all text neuron kinds on the registry.
pub fn register(registry: &mut Registry) {
    registry.register(
        NeuronKindInfo {
            id: "text.concat".into(),
            module: "text".into(),
            name: "Concat".into(),
            summary: "Joins two text values".into(),
            inputs: vec!["a".into(), "b".into()],
            outputs: vec!["text".into()],
        },
        Box::new(Concat),
    );
    registry.register(
        NeuronKindInfo {
            id: "text.upper".into(),
            module: "text".into(),
            name: "Upper".into(),
            summary: "Uppercases text".into(),
            inputs: vec!["text".into()],
            outputs: vec!["text".into()],
        },
        Box::new(Upper),
    );
}

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concat_joins_text() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new()
            .insert("a", Value::Atom(Atom::String("hi".into())))
            .insert("b", Value::Atom(Atom::String("!".into())));
        let out = reg.get("text.concat").unwrap().evaluate(&input).unwrap();
        assert_eq!(out.get("text").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("hi!"));
    }
}
// #endregion 🔖Tests
