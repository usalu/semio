//! ➕ Flow math module: neuron kinds for arithmetic.

use neural_engine::{Atom, Dictionary, EvalError, Function, NeuronKindInfo, Registry, Value};

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

// #region 🔖Sum
/// ∑ Sums all numbers in a list dictionary.
pub struct Sum;

impl Function for Sum {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let list = read_list(input, "list")?;
        let mut total = 0.0;
        for index in list_indices(list) {
            let value = list
                .get(&index.to_string())
                .and_then(|v| v.as_atom())
                .and_then(|a| a.as_f64())
                .ok_or_else(|| EvalError::MissingInput(index.to_string()))?;
            total += value;
        }
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(total))))
    }
}
// #endregion 🔖Sum

fn read_number(input: &Dictionary, key: &str) -> Result<f64, EvalError> {
    input
        .get(key)
        .and_then(|v| v.as_atom())
        .and_then(|a| a.as_f64())
        .ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_list<'a>(input: &'a Dictionary, key: &str) -> Result<&'a Dictionary, EvalError> {
    input
        .get(key)
        .and_then(|v| v.as_dictionary())
        .ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn list_indices(list: &Dictionary) -> Vec<usize> {
    let mut indices: Vec<usize> = list.keys().filter_map(|key| key.parse::<usize>().ok()).collect();
    indices.sort_unstable();
    indices
}

fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
}

/// 📦 Registers all math neuron kinds on the registry.
pub fn register(registry: &mut Registry) {
    registry.register(
        NeuronKindInfo {
            id: "math.add".into(),
            module: "math".into(),
            name: "Add".into(),
            summary: "Sums two numbers".into(),
            inputs: vec!["a".into(), "b".into()],
            outputs: vec!["number".into()],
            ..Default::default()
        },
        Box::new(Add),
    );
    registry.register(
        NeuronKindInfo {
            id: "math.multiply".into(),
            module: "math".into(),
            name: "Multiply".into(),
            summary: "Multiplies two numbers".into(),
            inputs: vec!["a".into(), "b".into()],
            outputs: vec!["number".into()],
            ..Default::default()
        },
        Box::new(Multiply),
    );
    registry.register(
        NeuronKindInfo {
            id: "math.passThrough".into(),
            module: "math".into(),
            name: "Pass Through".into(),
            summary: "Forwards a number".into(),
            inputs: vec!["number".into()],
            outputs: vec!["number".into()],
            ..Default::default()
        },
        Box::new(PassThrough),
    );
    registry.register(
        NeuronKindInfo {
            id: "math.sum".into(),
            module: "math".into(),
            name: "Sum".into(),
            summary: "Sums numbers in a list dictionary".into(),
            inputs: vec!["list".into()],
            outputs: vec!["number".into()],
            ..Default::default()
        },
        Box::new(Sum),
    );
}

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    use flow_module_wasm::{build_manifest_json, evaluate_json, FlowModuleCommandV1, FlowModuleSettingV1};

    #[test]
    fn add_sums_inputs() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new().insert("a", Value::Atom(Atom::Decimal(3.0))).insert("b", Value::Atom(Atom::Decimal(1.1)));
        let out = reg.get("math.add").unwrap().evaluate(&input).unwrap();
        assert_eq!(out.get("number").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(4.1));
    }

    #[test]
    fn manifest_lists_math_kinds() {
        let json = build_manifest_json(
            "math",
            "Math",
            "0.1.0",
            &module_registry(),
            vec!["onStartup".into()],
            vec![],
            vec![FlowModuleCommandV1 { id: "math.showHelp".into(), title: "Math: Show Help".into() }],
            vec![FlowModuleSettingV1 {
                id: "math.defaultPrecision".into(),
                setting_type: "number".into(),
                default: serde_json::json!(1),
                description: "Decimal places for number preview".into(),
            }],
        );
        assert!(json.contains("flow.module/v1"));
        assert!(json.contains("math.add"));
    }

    #[test]
    fn evaluate_json_adds_numbers() {
        let reg = module_registry();
        let input = Dictionary::new().insert("a", Value::Atom(Atom::Decimal(2.0))).insert("b", Value::Atom(Atom::Decimal(1.0)));
        let out_json = evaluate_json(&reg, "math.add", &serde_json::to_string(&input).unwrap());
        let out: Dictionary = serde_json::from_str(&out_json).unwrap();
        assert_eq!(out.get("number").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(3.0));
    }

    #[test]
    fn sum_totals_list_numbers() {
        let mut reg = Registry::new();
        register(&mut reg);
        let list = Dictionary::new()
            .insert("0", Value::Atom(Atom::Decimal(1.0)))
            .insert("1", Value::Atom(Atom::Decimal(2.5)))
            .insert("2", Value::Atom(Atom::Decimal(3.0)));
        let input = Dictionary::new().insert("list", Value::Dictionary(list));
        let out = reg.get("math.sum").unwrap().evaluate(&input).unwrap();
        assert_eq!(out.get("number").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(6.5));
    }
}
// #endregion 🔖Tests

// #region 🔖WasmExt
#[cfg(target_arch = "wasm32")]
mod wasm_ext {
    use super::module_registry;
    use flow_module_wasm::{build_manifest_json, command_json, evaluate_json, FlowModuleCommandV1, FlowModuleSettingV1};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn manifest() -> String {
        build_manifest_json(
            "math",
            "Math",
            "0.1.0",
            &module_registry(),
            vec!["onStartup".into()],
            vec![],
            vec![FlowModuleCommandV1 { id: "math.showHelp".into(), title: "Math: Show Help".into() }],
            vec![FlowModuleSettingV1 {
                id: "math.defaultPrecision".into(),
                setting_type: "number".into(),
                default: serde_json::json!(1),
                description: "Decimal places for number preview".into(),
            }],
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
