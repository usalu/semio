//! 📝 Flow text module: operators for text dictionaries.

use neural_engine::{channel_output, Atom, ChannelSpec, Dictionary, EvalError, Operation, OperatorImpl, OperatorInfo, Registry, Value};

// #region 🔖Concat
/// 🔗 Joins two text inputs.
pub struct Concat;

impl Operation for Concat {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("text", text_dictionary(format!("{}{}", read_channel_text(input, "a")?, read_channel_text(input, "b")?))))
    }
}
// #endregion 🔖Concat

// #region 🔖Upper
/// 🔠 Uppercases a text input.
pub struct Upper;

impl Operation for Upper {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("text", text_dictionary(read_channel_text(input, "text")?.to_uppercase())))
    }
}
// #endregion 🔖Upper

// #region 🔖Helpers
fn text_dictionary(value: String) -> Dictionary {
    Dictionary::with_schema("text").insert("value", Value::Atom(Atom::String(value)))
}

fn read_channel_text(input: &Dictionary, key: &str) -> Result<String, EvalError> {
    input.get(key).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_str()).map(str::to_string).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn text_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::text_default(id, "", &[operator_id])
}

fn info(id: &str, name: &str, summary: &str, inputs: Vec<ChannelSpec>, output: ChannelSpec) -> OperatorInfo {
    OperatorInfo { id: id.into(), module: "text".into(), name: name.into(), abbreviation: name.into(), icon: "emoji:📝".into(), summary: summary.into(), inputs, outputs: vec![output], ..Default::default() }
}

#[cfg(any(test, target_arch = "wasm32"))]
fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
}
// #endregion 🔖Helpers

/// 📦 Registers all text operators.
pub fn register(registry: &mut Registry) {
    registry.register_operator(
        info("text.concat", "Concat", "Joins two text values", vec![text_channel("a", "text.concat"), text_channel("b", "text.concat")], ChannelSpec::named("T", "Txt", "text", "JoinedText")),
        vec![OperatorImpl { schemas: vec!["text".into(), "text".into()], operation: Box::new(Concat) }],
        &["text"],
    );
    registry.register_operator(
        info("text.upper", "Upper", "Uppercases text", vec![text_channel("text", "text.upper")], ChannelSpec::named("T", "Txt", "text", "UppercasedText")),
        vec![OperatorImpl { schemas: vec!["text".into()], operation: Box::new(Upper) }],
        &["text"],
    );
    registry.finalize();
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
        let input = Dictionary::new().insert("a", Value::Dictionary(text_dictionary("hi".into()))).insert("b", Value::Dictionary(text_dictionary("!".into())));
        let out = reg.dispatch("text.concat", &input).unwrap();
        let text = out.get("text").and_then(|v| v.as_dictionary()).expect("text channel");
        assert_eq!(text.schema(), Some("text"));
        assert_eq!(text.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("hi!"));
    }

    #[test]
    fn manifest_lists_text_operators() {
        let json = build_manifest_json("text", "Text", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![FlowModuleCommandV1 { id: "text.showHelp".into(), title: "Text: Show Help".into() }], vec![]);
        assert!(json.contains("text.concat"));
        assert!(json.contains("\"operators\""));
    }

    #[test]
    fn evaluate_json_uppercases_text() {
        let input = Dictionary::new().insert("text", Value::Dictionary(text_dictionary("hi".into())));
        let out_json = evaluate_json(&module_registry(), "text.upper", &serde_json::to_string(&input).unwrap());
        let out: Dictionary = serde_json::from_str(&out_json).unwrap();
        let text = out.get("text").and_then(|v| v.as_dictionary()).expect("text channel");
        assert_eq!(text.schema(), Some("text"));
        assert_eq!(text.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("HI"));
    }
}
// #endregion 🔖Tests

// #region 🔖WasmExt
#[cfg(all(target_arch = "wasm32", feature = "standalone-wasm"))]
mod wasm_ext {
    use super::module_registry;
    use flow_module_wasm::{build_manifest_json, command_json, evaluate_json, FlowModuleCommandV1};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn manifest() -> String {
        build_manifest_json("text", "Text", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![FlowModuleCommandV1 { id: "text.showHelp".into(), title: "Text: Show Help".into() }], vec![])
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
