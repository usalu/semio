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

fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
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
            ..Default::default()
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
            ..Default::default()
        },
        Box::new(Upper),
    );
}

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    use flow_module_wasm::{build_manifest_json, evaluate_json, FlowModuleCommandV1};

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

    #[test]
    fn manifest_lists_text_kinds() {
        let json = build_manifest_json(
            "text",
            "Text",
            "0.1.0",
            &module_registry(),
            vec!["onStartup".into()],
            vec![],
            vec![FlowModuleCommandV1 { id: "text.showHelp".into(), title: "Text: Show Help".into() }],
            vec![],
        );
        assert!(json.contains("text.concat"));
    }

    #[test]
    fn evaluate_json_uppercases_text() {
        let reg = module_registry();
        let input = Dictionary::new().insert("text", Value::Atom(Atom::String("hi".into())));
        let out_json = evaluate_json(&reg, "text.upper", &serde_json::to_string(&input).unwrap());
        let out: Dictionary = serde_json::from_str(&out_json).unwrap();
        assert_eq!(out.get("text").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("HI"));
    }
}
// #endregion 🔖Tests

// #region 🔖WasmExt
#[cfg(target_arch = "wasm32")]
mod wasm_ext {
    use super::module_registry;
    use flow_module_wasm::{build_manifest_json, command_json, evaluate_json, FlowModuleCommandV1};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn manifest() -> String {
        build_manifest_json(
            "text",
            "Text",
            "0.1.0",
            &module_registry(),
            vec!["onStartup".into()],
            vec![],
            vec![FlowModuleCommandV1 { id: "text.showHelp".into(), title: "Text: Show Help".into() }],
            vec![],
        )
    }

    #[wasm_bindgen]
    pub fn evaluate(kind_id: &str, input_json: &str) -> String {
        evaluate_json(&module_registry(), kind_id, input_json)
    }

    #[wasm_bindgen]
    pub fn command(command_id: &str, args_json: &str) -> String {
        command_json(command_id, args_json)
    }

    #[wasm_bindgen]
    pub fn activate() {}

    #[wasm_bindgen]
    pub fn deactivate() {}
}
// #endregion 🔖WasmExt
