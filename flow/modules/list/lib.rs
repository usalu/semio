//! 📋 Flow list module: list dictionary operators.

use neural_engine::{Atom, ChannelSpec, Dictionary, EvalError, Operation, OperatorImpl, OperatorInfo, Registry, Value, VariadicSpec};

// #region 🔖Empty
/// 📭 Creates an empty list dictionary.
pub struct Empty;

impl Operation for Empty {
    fn evaluate(&self, _input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(Dictionary::with_schema("list"))
    }
}
// #endregion 🔖Empty

// #region 🔖Pack
/// 📦 Wraps input as a list dictionary.
pub struct Pack;

impl Operation for Pack {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let mut out = Dictionary::with_schema("list");
        if let Some(value) = input.get("*") {
            out = out.insert("0", value.clone());
        } else {
            for key in input.keys() {
                if let Some(value) = input.get(key) {
                    out = out.insert(key.clone(), value.clone());
                }
            }
        }
        Ok(out)
    }
}
// #endregion 🔖Pack

// #region 🔖Get
/// 🔍 Reads a value by index from a list dictionary.
pub struct Get;

impl Operation for Get {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let list = read_list(input, "list")?;
        let index = read_number(input, "index")? as usize;
        let wrap = read_bool(input, "wrap").unwrap_or(false);
        let indices = list_indices(&list);
        if indices.is_empty() {
            return Err(EvalError::InvalidInput("empty list".into()));
        }
        let resolved = if wrap {
            indices[index % indices.len()]
        } else {
            indices.get(index).copied().ok_or_else(|| EvalError::InvalidInput("index out of range".into()))?
        };
        list.get(&resolved.to_string()).cloned().ok_or_else(|| EvalError::MissingInput(resolved.to_string()))
            .map(|value| match value {
                Value::Dictionary(dict) => dict,
                other => Dictionary::new().insert("value", other),
            })
    }
}
// #endregion 🔖Get

// #region 🔖Set
/// ✏️ Replaces a value at an index.
pub struct Set;

impl Operation for Set {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let list = read_list(input, "list")?;
        let index = read_number(input, "index")? as usize;
        let value = input.get("value").cloned().ok_or_else(|| EvalError::MissingInput("value".into()))?;
        Ok(list.insert(index.to_string(), value))
    }
}
// #endregion 🔖Set

// #region 🔖Append
/// ➕ Appends a value at the next index.
pub struct Append;

impl Operation for Append {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let list = read_list(input, "list")?;
        let value = input.get("value").cloned().ok_or_else(|| EvalError::MissingInput("value".into()))?;
        let next = list_indices(&list).len();
        Ok(list.insert(next.to_string(), value))
    }
}
// #endregion 🔖Append

// #region 🔖Size
/// 📏 Reports the number of indexed elements.
pub struct Size;

impl Operation for Size {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let list = read_list(input, "list")?;
        Ok(number_dictionary(list_indices(&list).len() as f64))
    }
}
// #endregion 🔖Size

// #region 🔖Remove
/// 🗑️ Removes an index and reindexes.
pub struct Remove;

impl Operation for Remove {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let list = read_list(input, "list")?;
        let index = read_number(input, "index")? as usize;
        Ok(remove_list_index(&list, index))
    }
}
// #endregion 🔖Remove

// #region 🔖Range
/// 📐 Builds an arithmetic sequence list.
pub struct Range;

impl Operation for Range {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let start = read_number(input, "start")?;
        let step = read_number(input, "step").unwrap_or(1.0);
        let count = read_number(input, "count").unwrap_or(1.0).max(0.0) as usize;
        let mut out = Dictionary::with_schema("list");
        for index in 0..count {
            out = out.insert(index.to_string(), Value::Dictionary(number_dictionary(start + step * index as f64)));
        }
        Ok(out)
    }
}
// #endregion 🔖Range

// #region 🔖Reverse
/// 🔁 Reverses indexed list elements.
pub struct Reverse;

impl Operation for Reverse {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let list = read_list(input, "list")?;
        let indices = list_indices(&list);
        let mut out = Dictionary::with_schema("list");
        for (next, index) in indices.into_iter().rev().enumerate() {
            if let Some(value) = list.get(&index.to_string()) {
                out = out.insert(next.to_string(), value.clone());
            }
        }
        Ok(out)
    }
}
// #endregion 🔖Reverse

// #region 🔖Helpers
fn number_dictionary(value: f64) -> Dictionary {
    Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(value)))
}

fn read_list<'a>(input: &'a Dictionary, key: &str) -> Result<Dictionary, EvalError> {
    input
        .get(key)
        .and_then(|value| value.as_dictionary())
        .filter(|dict| dict.schema() == Some("list"))
        .cloned()
        .ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_number(input: &Dictionary, key: &str) -> Result<f64, EvalError> {
    input
        .get(key)
        .and_then(|value| value.as_dictionary())
        .and_then(|dict| dict.get("value"))
        .and_then(|value| value.as_atom())
        .and_then(|atom| atom.as_f64())
        .ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_bool(input: &Dictionary, key: &str) -> Result<bool, EvalError> {
    input
        .get(key)
        .and_then(|value| value.as_dictionary())
        .and_then(|dict| dict.get("value"))
        .and_then(|value| value.as_atom())
        .and_then(|atom| atom.as_bool())
        .ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn list_indices(list: &Dictionary) -> Vec<usize> {
    let mut indices: Vec<usize> = list.keys().filter_map(|key| key.parse::<usize>().ok()).collect();
    indices.sort_unstable();
    indices
}

fn remove_list_index(list: &Dictionary, remove_at: usize) -> Dictionary {
    let mut out = Dictionary::with_schema("list");
    let mut next = 0usize;
    for index in list_indices(list) {
        if index == remove_at {
            continue;
        }
        if let Some(value) = list.get(&index.to_string()) {
            out = out.insert(next.to_string(), value.clone());
            next += 1;
        }
    }
    out
}

fn list_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::list(id, &[operator_id])
}

fn number_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::number_default(id, 0.0, &[operator_id])
}

fn out_channel() -> ChannelSpec {
    ChannelSpec::provides("out", vec![])
}

fn info(id: &str, name: &str, summary: &str, inputs: Vec<ChannelSpec>, output: ChannelSpec) -> OperatorInfo {
    OperatorInfo {
        id: id.into(),
        module: "list".into(),
        name: name.into(),
        abbreviation: name.into(),
        icon: "emoji:📋".into(),
        summary: summary.into(),
        inputs,
        outputs: vec![output],
        ..Default::default()
    }
}

fn register_simple(registry: &mut Registry, info: OperatorInfo, operation: Box<dyn Operation>, schemas: Vec<&str>, produces: &[&str]) {
    registry.register_operator(
        info,
        vec![OperatorImpl {
            schemas: schemas.into_iter().map(str::to_string).collect(),
            operation,
        }],
        produces,
    );
}

#[cfg(any(test, target_arch = "wasm32"))]
fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
}
// #endregion 🔖Helpers

/// 📦 Registers all list operators.
pub fn register(registry: &mut Registry) {
    register_simple(registry, info("list.empty", "Empty", "Creates an empty list", vec![], out_channel()), Box::new(Empty), vec![], &["list"]);
    register_simple(registry, info("list.pack", "Pack", "Wraps input as a list dictionary", vec![ChannelSpec::wildcard()], out_channel()), Box::new(Pack), vec![], &["list"]);
    register_simple(
        registry,
        info(
            "list.get",
            "Get",
            "Reads a value by index",
            vec![list_channel("list", "list.get"), number_channel("index", "list.get"), ChannelSpec::boolean_default("wrap", false, &["list.get"])],
            ChannelSpec::any("out"),
        ),
        Box::new(Get),
        vec!["list", "number", "boolean"],
        &["value"],
    );
    register_simple(
        registry,
        info("list.set", "Set", "Replaces a value at an index", vec![list_channel("list", "list.set"), number_channel("index", "list.set"), ChannelSpec::any("value")], out_channel()),
        Box::new(Set),
        vec![],
        &["list"],
    );
    register_simple(
        registry,
        info("list.append", "Append", "Appends a value at the next index", vec![list_channel("list", "list.append"), ChannelSpec::any("value")], out_channel()),
        Box::new(Append),
        vec![],
        &["list"],
    );
    register_simple(registry, info("list.size", "Size", "Reports the number of indexed elements", vec![list_channel("list", "list.size")], out_channel()), Box::new(Size), vec!["list"], &["number"]);
    register_simple(
        registry,
        info("list.remove", "Remove", "Removes an index and reindexes", vec![list_channel("list", "list.remove"), number_channel("index", "list.remove")], out_channel()),
        Box::new(Remove),
        vec!["list", "number"],
        &["list"],
    );
    register_simple(
        registry,
        info(
            "list.range",
            "Range",
            "Builds an arithmetic sequence list",
            vec![number_channel("start", "list.range"), ChannelSpec::number_default("step", 1.0, &["list.range"]), ChannelSpec::number_default("count", 1.0, &["list.range"])],
            out_channel(),
        ),
        Box::new(Range),
        vec!["number", "number", "number"],
        &["list"],
    );
    register_simple(registry, info("list.reverse", "Reverse", "Reverses indexed list elements", vec![list_channel("list", "list.reverse")], out_channel()), Box::new(Reverse), vec!["list"], &["list"]);
    registry.finalize();
}

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use flow_module_wasm::{build_manifest_json, evaluate_json, FlowModuleCommandV1};

    fn sample_list() -> Dictionary {
        Dictionary::with_schema("list")
            .insert("0", Value::Dictionary(number_dictionary(1.0)))
            .insert("1", Value::Dictionary(number_dictionary(2.0)))
            .insert("2", Value::Dictionary(number_dictionary(3.0)))
    }

    #[test]
    fn empty_creates_list() {
        let mut reg = Registry::new();
        register(&mut reg);
        let out = reg.dispatch("list.empty", &Dictionary::new()).unwrap();
        assert_eq!(out.schema(), Some("list"));
    }

    #[test]
    fn get_reads_index() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new()
            .insert("list", Value::Dictionary(sample_list()))
            .insert("index", Value::Dictionary(number_dictionary(1.0)))
            .insert("wrap", Value::Dictionary(Dictionary::with_schema("boolean").insert("value", Value::Atom(Atom::Boolean(false)))));
        let out = reg.dispatch("list.get", &input).unwrap();
        assert_eq!(out.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(2.0));
    }

    #[test]
    fn append_adds_next_index() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new()
            .insert("list", Value::Dictionary(sample_list()))
            .insert("value", Value::Dictionary(number_dictionary(4.0)));
        let out = reg.dispatch("list.append", &input).unwrap();
        assert_eq!(out.get("3").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(4.0));
    }

    #[test]
    fn manifest_lists_operators() {
        let mut reg = Registry::new();
        register(&mut reg);
        let json = build_manifest_json("list", "List", "0.1.0", &reg, vec!["onStartup".into()], vec![], vec![FlowModuleCommandV1 { id: "list.test".into(), title: "Test".into() }], vec![]);
        assert!(json.contains("list.get"));
        assert!(json.contains("operators"));
    }

    #[test]
    fn evaluate_json_round_trips() {
        let reg = module_registry();
        let input = Dictionary::new().insert("list", Value::Dictionary(sample_list()));
        let out_json = evaluate_json(&reg, "list.size", &serde_json::to_string(&input).unwrap());
        assert!(out_json.contains("\"value\""));
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
            "list",
            "List",
            "0.1.0",
            &module_registry(),
            vec!["onStartup".into()],
            vec![],
            vec![FlowModuleCommandV1 { id: "list.showHelp".into(), title: "List: Show Help".into() }],
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
