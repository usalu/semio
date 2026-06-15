//! 🧱 Flow core module: schema constructors for primitive dictionaries.

use neural_engine::{channel_output, Atom, ChannelSpec, Dictionary, EvalError, FieldSpec, Operation, OperatorImpl, OperatorInfo, Registry, Schema, Value, ValueType};

// #region 🔖Number
/// 🔢 Emits a number dictionary.
pub struct Number;

impl Operation for Number {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("number", number_dictionary(read_number(input, "value").or_else(|_| read_number(input, "number"))?)))
    }
}
// #endregion 🔖Number

// #region 🔖Text
/// 📝 Emits a text dictionary.
pub struct Text;

impl Operation for Text {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("text", text_dictionary(read_text(input, "value").or_else(|_| read_text(input, "text"))?)))
    }
}
// #endregion 🔖Text

// #region 🔖Boolean
/// 🔀 Emits a boolean dictionary.
pub struct Boolean;

impl Operation for Boolean {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("boolean", boolean_dictionary(read_bool(input, "value").or_else(|_| read_bool(input, "boolean")).unwrap_or(false))))
    }
}
// #endregion 🔖Boolean

// #region 🔖Image
/// 🖼️ Emits an image dictionary.
pub struct Image;

impl Operation for Image {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("image", Dictionary::with_schema("image").insert("dataUrl", Value::Atom(Atom::String(read_text(input, "dataUrl").unwrap_or_default())))))
    }
}
// #endregion 🔖Image

// #region 🔖Variable
/// 🔣 Forwards a named dictionary channel unchanged.
pub struct VariableRelay;

impl Operation for VariableRelay {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let name = read_text(input, "name")?;
        let schema = read_text(input, "schema").unwrap_or_else(|_| "dictionary".into());
        let payload = input.get(&name).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(name.clone()))?;
        if let Some(actual) = payload.schema() {
            if actual != schema.as_str() {
                return Err(EvalError::InvalidInput(format!("expected schema {schema}, got {actual}")));
            }
        }
        Ok(channel_output(&name, payload.clone()))
    }
}
// #endregion 🔖Variable

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
    input.get(key).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_bool(input: &Dictionary, key: &str) -> Result<bool, EvalError> {
    input.get(key).and_then(|v| v.as_atom()).and_then(|a| a.as_bool()).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_text(input: &Dictionary, key: &str) -> Result<String, EvalError> {
    input.get(key).and_then(|v| v.as_atom()).and_then(|a| a.as_str()).map(str::to_string).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn schema(id: &str, name: &str, summary: &str, fields: Vec<FieldSpec>) -> Schema {
    Schema { id: id.into(), module: "core".into(), name: name.into(), icon: "emoji:🧱".into(), summary: summary.into(), fields }
}

fn operator(id: &str, name: &str, summary: &str, outputs: Vec<ChannelSpec>, operation: Box<dyn Operation>) -> (OperatorInfo, Vec<OperatorImpl>) {
    (
        OperatorInfo { id: id.into(), module: "core".into(), name: name.into(), abbreviation: name.into(), icon: "emoji:🧱".into(), summary: summary.into(), inputs: vec![], outputs, ..Default::default() },
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

    let (info, implementations) = operator("core.number", "Number", "Produces a number dictionary", vec![ChannelSpec::named("N", "Num", "number", "Number")], Box::new(Number));
    registry.register_operator(info, implementations, &["number"]);
    let (info, implementations) = operator("core.text", "Text", "Produces a text dictionary", vec![ChannelSpec::named("T", "Txt", "text", "Text")], Box::new(Text));
    registry.register_operator(info, implementations, &["text"]);
    let (info, implementations) = operator("core.boolean", "Bool", "Produces a boolean dictionary", vec![ChannelSpec::named("B", "Boo", "boolean", "Boolean")], Box::new(Boolean));
    registry.register_operator(info, implementations, &["boolean"]);
    let (info, implementations) = operator("core.image", "Image", "Produces an image dictionary", vec![ChannelSpec::named("I", "Img", "image", "Image")], Box::new(Image));
    registry.register_operator(info, implementations, &["image"]);
    registry.register_operator(
        OperatorInfo {
            id: "core.variable".into(),
            module: "core".into(),
            name: "Variable".into(),
            abbreviation: "Var".into(),
            icon: "emoji:🔣".into(),
            summary: "Relays a named typed dictionary".into(),
            inputs: vec![ChannelSpec::wildcard()],
            outputs: vec![ChannelSpec::wildcard()],
            ..Default::default()
        },
        vec![OperatorImpl { schemas: vec![], operation: Box::new(VariableRelay) }],
        &[],
    );
    registry.finalize();
}

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use flow_module_wasm::{build_manifest_json, evaluate_json};

    #[test]
    fn variable_relay_forwards_named_channel() {
        let mut registry = Registry::new();
        register(&mut registry);
        let input = Dictionary::new()
            .insert("name", Value::Atom(Atom::String("width".into())))
            .insert("schema", Value::Atom(Atom::String("number".into())))
            .insert("width", Value::Dictionary(number_dictionary(2.0)));
        let out = registry.dispatch("core.variable", &input).unwrap();
        let width = out.get("width").and_then(|v| v.as_dictionary()).expect("width channel");
        assert_eq!(width.schema(), Some("number"));
    }

    #[test]
    fn number_emits_schema_dictionary() {
        let mut registry = Registry::new();
        register(&mut registry);
        let out = registry.dispatch("core.number", &Dictionary::new().insert("value", Value::Atom(Atom::Decimal(2.5)))).unwrap();
        let number = out.get("number").and_then(|v| v.as_dictionary()).expect("number channel");
        assert_eq!(number.schema(), Some("number"));
        assert_eq!(number.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(2.5));
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
        let text = out.get("text").and_then(|v| v.as_dictionary()).expect("text channel");
        assert_eq!(text.schema(), Some("text"));
        assert_eq!(text.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("hi"));
    }
}
// #endregion 🔖Tests

// #region 🔖WasmExt
#[cfg(all(target_arch = "wasm32", feature = "standalone-wasm"))]
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
