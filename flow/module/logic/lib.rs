//! 🔀 Flow logic module: boolean operators over schema dictionaries.

use neural_engine::{channel_output, Atom, ChannelSpec, Dictionary, EvalError, Operation, OperatorImpl, OperatorInfo, Registry, Value};

// #region 🔖Greater
/// 📈 Compares two numbers.
pub struct Greater;

impl Operation for Greater {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("boolean", boolean_dictionary(read_channel_number(input, "a")? > read_channel_number(input, "b")?)))
    }
}
// #endregion 🔖Greater

// #region 🔖Not
/// 🔄 Inverts a boolean.
pub struct Not;

impl Operation for Not {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("boolean", boolean_dictionary(!read_channel_bool(input, "boolean")?)))
    }
}
// #endregion 🔖Not

// #region 🔖Helpers
fn boolean_dictionary(value: bool) -> Dictionary {
    Dictionary::with_schema("boolean").insert("value", Value::Atom(Atom::Boolean(value)))
}

fn read_channel_number(input: &Dictionary, key: &str) -> Result<f64, EvalError> {
    input.get(key).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_channel_bool(input: &Dictionary, key: &str) -> Result<bool, EvalError> {
    input.get(key).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_bool()).ok_or_else(|| EvalError::MissingInput(key.into()))
}

#[cfg(test)]
fn number_dictionary(value: f64) -> Dictionary {
    Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(value)))
}

fn number_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::number_default(id, 0.0, &[operator_id])
}

fn boolean_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::boolean_default(id, false, &[operator_id])
}

fn info(id: &str, name: &str, summary: &str, inputs: Vec<ChannelSpec>, output: ChannelSpec) -> OperatorInfo {
    OperatorInfo { id: id.into(), module: "logic".into(), name: name.into(), abbreviation: name.into(), icon: "emoji:🔀".into(), summary: summary.into(), inputs, outputs: vec![output], ..Default::default() }
}

#[cfg(any(test, target_arch = "wasm32"))]
fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
}
// #endregion 🔖Helpers

/// 📦 Registers all logic operators.
pub fn register(registry: &mut Registry) {
    registry.register_operator(
        info("logic.greater", "Greater", "True when a > b", vec![number_channel("a", "logic.greater"), number_channel("b", "logic.greater")], ChannelSpec::named("B", "Boo", "boolean", "Greater")),
        vec![OperatorImpl { schemas: vec!["number".into(), "number".into()], operation: Box::new(Greater) }],
        &["boolean"],
    );
    registry.register_operator(
        info("logic.not", "Not", "Inverts a boolean", vec![boolean_channel("boolean", "logic.not")], ChannelSpec::named("B", "Boo", "boolean", "Negated")),
        vec![OperatorImpl { schemas: vec!["boolean".into()], operation: Box::new(Not) }],
        &["boolean"],
    );
    registry.finalize();
}

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use flow_module_wasm::{build_manifest_json, evaluate_json, FlowModuleCommandV1};

    #[test]
    fn greater_compares_numbers() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new().insert("a", Value::Dictionary(number_dictionary(5.0))).insert("b", Value::Dictionary(number_dictionary(2.0)));
        let out = reg.dispatch("logic.greater", &input).unwrap();
        let boolean = out.get("boolean").and_then(|v| v.as_dictionary()).expect("boolean channel");
        assert_eq!(boolean.schema(), Some("boolean"));
        assert_eq!(boolean.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_bool()), Some(true));
    }

    #[test]
    fn manifest_lists_logic_operators() {
        let json = build_manifest_json("logic", "Logic", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![FlowModuleCommandV1 { id: "logic.showHelp".into(), title: "Logic: Show Help".into() }], vec![]);
        assert!(json.contains("logic.greater"));
    }

    #[test]
    fn evaluate_json_greater() {
        let input = Dictionary::new().insert("a", Value::Dictionary(number_dictionary(5.0))).insert("b", Value::Dictionary(number_dictionary(2.0)));
        let out_json = evaluate_json(&module_registry(), "logic.greater", &serde_json::to_string(&input).unwrap());
        let out: Dictionary = serde_json::from_str(&out_json).unwrap();
        let boolean = out.get("boolean").and_then(|v| v.as_dictionary()).expect("boolean channel");
        assert_eq!(boolean.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_bool()), Some(true));
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
        build_manifest_json("logic", "Logic", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![FlowModuleCommandV1 { id: "logic.showHelp".into(), title: "Logic: Show Help".into() }], vec![])
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
