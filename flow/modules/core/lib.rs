//! 🧱 Flow core module: schema constructors for primitive dictionaries.

use neural_engine::{Atom, ChannelSpec, Dictionary, EvalError, FieldSpec, Operation, OperatorImpl, OperatorInfo, Registry, Schema, Value, ValueType};

// #region 🔖Number
/// 🔢 Emits a number dictionary.
pub struct Number;

impl Operation for Number {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(number_dictionary(read_number(input, "value").or_else(|_| read_number(input, "number"))?))
    }
}
// #endregion 🔖Number

// #region 🔖Text
/// 📝 Emits a text dictionary.
pub struct Text;

impl Operation for Text {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(text_dictionary(read_text(input, "value").or_else(|_| read_text(input, "text"))?))
    }
}
// #endregion 🔖Text

// #region 🔖Boolean
/// 🔀 Emits a boolean dictionary.
pub struct Boolean;

impl Operation for Boolean {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(boolean_dictionary(read_bool(input, "value").or_else(|_| read_bool(input, "boolean")).unwrap_or(false)))
    }
}
// #endregion 🔖Boolean

// #region 🔖Image
/// 🖼️ Emits an image dictionary.
pub struct Image;

impl Operation for Image {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(Dictionary::with_schema("image").insert("dataUrl", Value::Atom(Atom::String(read_text(input, "dataUrl").unwrap_or_default()))))
    }
}
// #endregion 🔖Image

// #region 🔖Helpers
pub fn number_dictionary(value: f64) -> Dictionary {
    Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(value)))
}

pub fn text_dictionary(value: String) -> Dictionary {
    Dictionary::with_schema("text").insert("value", Value::Atom(Atom::String(value)))
}

pub fn boolean_dictionary(value: bool) -> Dictionary {
    Dictionary::with_schema("boolean").insert("value", Value::Atom(Atom::Boolean(value)))
}

fn read_number(input: &Dictionary, key: &str) -> Result<f64, EvalError> {
    input
        .get(key)
        .and_then(|v| v.as_atom())
        .and_then(|a| a.as_f64())
        .ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_bool(input: &Dictionary, key: &str) -> Result<bool, EvalError> {
    input
        .get(key)
        .and_then(|v| v.as_atom())
        .and_then(|a| a.as_bool())
        .ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_text(input: &Dictionary, key: &str) -> Result<String, EvalError> {
    input
        .get(key)
        .and_then(|v| v.as_atom())
        .and_then(|a| a.as_str())
        .map(str::to_string)
        .ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn schema(id: &str, name: &str, summary: &str, fields: Vec<FieldSpec>) -> Schema {
    Schema {
        id: id.into(),
        module: "core".into(),
        name: name.into(),
        icon: "emoji:🧱".into(),
        summary: summary.into(),
        fields,
    }
}

fn operator(id: &str, name: &str, summary: &str, outputs: Vec<ChannelSpec>, operation: Box<dyn Operation>) -> (OperatorInfo, Vec<OperatorImpl>) {
    (
        OperatorInfo {
            id: id.into(),
            module: "core".into(),
            name: name.into(),
            abbreviation: name.into(),
            icon: "emoji:🧱".into(),
            summary: summary.into(),
            inputs: vec![],
            outputs,
            ..Default::default()
        },
        vec![OperatorImpl { schemas: vec![], operation }],
    )
}

#[cfg(any(test, target_arch = "wasm32"))]
fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
}
// #endregion 🔖Helpers

/// 📦 Registers core schemas and value operators.
pub fn register(registry: &mut Registry) {
    registry.register_schema(schema("number", "Number", "Decimal number", vec![FieldSpec::decimal_default("value", 0.0)]));
    registry.register_schema(schema("text", "Text", "Text value", vec![FieldSpec::new("value", ValueType::Text).with_default(Value::Atom(Atom::String(String::new())))]));
    registry.register_schema(schema("boolean", "Boolean", "Boolean value", vec![FieldSpec::new("value", ValueType::Boolean).with_default(Value::Atom(Atom::Boolean(false)))]));
    registry.register_schema(schema("list", "List", "Index-keyed dictionary list", vec![]));
    registry.register_schema(schema("dictionary", "Dictionary", "Arbitrary dictionary", vec![]));
    registry.register_schema(schema("image", "Image", "Image data URL", vec![FieldSpec::new("dataUrl", ValueType::Text).with_default(Value::Atom(Atom::String(String::new())))]));

    let (info, implementations) = operator("core.number", "Number", "Produces a number dictionary", vec![ChannelSpec::number("out")], Box::new(Number));
    registry.register_operator(info, implementations);
    let (info, implementations) = operator("core.text", "Text", "Produces a text dictionary", vec![ChannelSpec::new("out", ValueType::Schema("text".into()))], Box::new(Text));
    registry.register_operator(info, implementations);
    let (info, implementations) = operator("core.boolean", "Bool", "Produces a boolean dictionary", vec![ChannelSpec::new("out", ValueType::Schema("boolean".into()))], Box::new(Boolean));
    registry.register_operator(info, implementations);
    let (info, implementations) = operator("core.image", "Image", "Produces an image dictionary", vec![ChannelSpec::new("out", ValueType::Schema("image".into()))], Box::new(Image));
    registry.register_operator(info, implementations);
}

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use flow_module_wasm::{build_manifest_json, evaluate_json};

    #[test]
    fn number_emits_schema_dictionary() {
        let mut registry = Registry::new();
        register(&mut registry);
        let out = registry.dispatch("core.number", &Dictionary::new().insert("value", Value::Atom(Atom::Decimal(2.5)))).unwrap();
        assert_eq!(out.schema(), Some("number"));
        assert_eq!(out.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(2.5));
    }

    #[test]
    fn manifest_lists_schemas_and_operators() {
        let json = build_manifest_json("core", "Core", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![]);
        assert!(json.contains("\"schemas\""));
        assert!(json.contains("core.number"));
        assert!(json.contains("\"number\""));
    }

    #[test]
    fn evaluate_json_text() {
        let input = Dictionary::new().insert("value", Value::Atom(Atom::String("hi".into())));
        let out_json = evaluate_json(&module_registry(), "core.text", &serde_json::to_string(&input).unwrap());
        let out: Dictionary = serde_json::from_str(&out_json).unwrap();
        assert_eq!(out.schema(), Some("text"));
        assert_eq!(out.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("hi"));
    }
}
// #endregion 🔖Tests

// #region 🔖WasmExt
#[cfg(target_arch = "wasm32")]
mod wasm_ext {
    use super::module_registry;
    use flow_module_wasm::{build_manifest_json, command_json, evaluate_json};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn manifest() -> String {
        build_manifest_json("core", "Core", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![])
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
