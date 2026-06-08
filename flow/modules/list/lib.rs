//! 📋 Flow list module: index-keyed schema dictionaries as lists.

use neural_engine::{Atom, ChannelSpec, Dictionary, EvalError, Operation, OperatorImpl, OperatorInfo, Registry, Value, ValueType};

// #region 🔖Empty
/// 🆕 Creates an empty list dictionary.
pub struct Empty;

impl Operation for Empty {
    fn evaluate(&self, _input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(Dictionary::with_schema("list"))
    }
}
// #endregion 🔖Empty

// #region 🔖Pack
/// 📦 Wraps the input dictionary as a list value.
pub struct Pack;

impl Operation for Pack {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let mut list = Dictionary::with_schema("list");
        for key in input.keys() {
            if key == "$schema" {
                continue;
            }
            if let Some(value) = input.get(key) {
                list = list.insert(key.clone(), value.clone());
            }
        }
        Ok(list)
    }
}
// #endregion 🔖Pack

// #region 🔖Get
/// 🔍 Reads a list element by index.
pub struct Get;

impl Operation for Get {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let list = read_list(input, "list")?;
        let len = list_len(list);
        let index = resolve_list_index(input, "index", len, read_channel_bool(input, "wrap").unwrap_or(false))?;
        match list.get(&index.to_string()).cloned().ok_or_else(|| EvalError::MissingInput(index.to_string()))? {
            Value::Dictionary(dictionary) => Ok(dictionary),
            value => Ok(Dictionary::with_schema("dictionary").insert("value", value)),
        }
    }
}
// #endregion 🔖Get

// #region 🔖Set
/// ✏️ Replaces a list element at an index.
pub struct Set;

impl Operation for Set {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let list = read_list(input, "list")?;
        let index = read_channel_index(input, "index")?;
        let value = input.get("value").cloned().ok_or_else(|| EvalError::MissingInput("value".into()))?;
        Ok(list.clone().insert(index.to_string(), value))
    }
}
// #endregion 🔖Set

// #region 🔖Append
/// ➕ Appends a value at the next list index.
pub struct Append;

impl Operation for Append {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let list = read_list(input, "list")?;
        let value = input.get("value").cloned().ok_or_else(|| EvalError::MissingInput("value".into()))?;
        Ok(list.clone().insert(next_list_index(list).to_string(), value))
    }
}
// #endregion 🔖Append

// #region 🔖Size
/// 📏 Reports the number of indexed list elements.
pub struct Size;

impl Operation for Size {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(number_dictionary(list_len(read_list(input, "list")?) as f64))
    }
}
// #endregion 🔖Size

// #region 🔖Remove
/// 🗑️ Removes a list element and reindexes remaining items.
pub struct Remove;

impl Operation for Remove {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(remove_list_index(read_list(input, "list")?, read_channel_index(input, "index")?))
    }
}
// #endregion 🔖Remove

// #region 🔖Range
/// 📈 Builds an arithmetic sequence list.
pub struct Range;

impl Operation for Range {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let start = read_channel_number(input, "start")?;
        let step = read_channel_number(input, "step")?;
        let count = read_channel_index(input, "count")?;
        let mut list = Dictionary::with_schema("list");
        for index in 0..count {
            list = list.insert(index.to_string(), Value::Dictionary(number_dictionary(start + step * index as f64)));
        }
        Ok(list)
    }
}
// #endregion 🔖Range

// #region 🔖Reverse
/// 🔁 Reverses indexed list elements.
pub struct Reverse;

impl Operation for Reverse {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let list = read_list(input, "list")?;
        let indices = list_indices(list);
        let mut out = Dictionary::with_schema("list");
        for (next, index) in indices.iter().rev().enumerate() {
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

fn read_list<'a>(input: &'a Dictionary, key: &str) -> Result<&'a Dictionary, EvalError> {
    input.get(key).and_then(|v| v.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_channel_number(input: &Dictionary, key: &str) -> Result<f64, EvalError> {
    input
        .get(key)
        .and_then(|v| v.as_dictionary())
        .and_then(|d| d.get("value"))
        .and_then(|v| v.as_atom())
        .and_then(|a| a.as_f64())
        .ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_channel_bool(input: &Dictionary, key: &str) -> Result<bool, EvalError> {
    input
        .get(key)
        .and_then(|v| v.as_dictionary())
        .and_then(|d| d.get("value"))
        .and_then(|v| v.as_atom())
        .and_then(|a| a.as_bool())
        .ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_channel_index(input: &Dictionary, key: &str) -> Result<usize, EvalError> {
    Ok(read_channel_number(input, key)? as usize)
}

fn list_indices(list: &Dictionary) -> Vec<usize> {
    let mut indices: Vec<usize> = list.keys().filter_map(|key| key.parse::<usize>().ok()).collect();
    indices.sort_unstable();
    indices
}

fn list_len(list: &Dictionary) -> usize {
    list_indices(list).len()
}

fn next_list_index(list: &Dictionary) -> usize {
    list_indices(list).last().map(|index| index + 1).unwrap_or(0)
}

fn resolve_list_index(input: &Dictionary, key: &str, len: usize, wrap: bool) -> Result<usize, EvalError> {
    let raw = read_channel_number(input, key)?;
    if len == 0 {
        return Err(EvalError::InvalidInput("empty list".into()));
    }
    if wrap {
        return Ok((raw.floor() as i64).rem_euclid(len as i64) as usize);
    }
    let index = raw as usize;
    if index >= len {
        return Err(EvalError::MissingInput(index.to_string()));
    }
    Ok(index)
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

fn list_channel(id: &str) -> ChannelSpec {
    ChannelSpec::list(id)
}

fn number_channel(id: &str) -> ChannelSpec {
    ChannelSpec::number_default(id, 0.0)
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

fn register_simple(registry: &mut Registry, info: OperatorInfo, operation: Box<dyn Operation>, schemas: Vec<&str>) {
    registry.register_operator(info, vec![OperatorImpl { schemas: schemas.into_iter().map(str::to_string).collect(), operation }]);
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
    register_simple(registry, info("list.empty", "Empty", "Creates an empty list", vec![], ChannelSpec::new("out", ValueType::Schema("list".into()))), Box::new(Empty), vec![]);
    register_simple(registry, info("list.pack", "Pack", "Wraps input as a list dictionary", vec![ChannelSpec::wildcard()], ChannelSpec::new("out", ValueType::Schema("list".into()))), Box::new(Pack), vec![]);
    register_simple(registry, info("list.get", "Get", "Reads a value by index", vec![list_channel("list"), number_channel("index"), ChannelSpec::boolean_default("wrap", false)], ChannelSpec::value("out")), Box::new(Get), vec!["list", "number", "boolean"]);
    register_simple(registry, info("list.set", "Set", "Replaces a value at an index", vec![list_channel("list"), number_channel("index"), ChannelSpec::value("value")], ChannelSpec::new("out", ValueType::Schema("list".into()))), Box::new(Set), vec![]);
    register_simple(registry, info("list.append", "Append", "Appends a value at the next index", vec![list_channel("list"), ChannelSpec::value("value")], ChannelSpec::new("out", ValueType::Schema("list".into()))), Box::new(Append), vec![]);
    register_simple(registry, info("list.size", "Size", "Reports the number of indexed elements", vec![list_channel("list")], ChannelSpec::number("out")), Box::new(Size), vec!["list"]);
    register_simple(registry, info("list.remove", "Remove", "Removes an index and reindexes", vec![list_channel("list"), number_channel("index")], ChannelSpec::new("out", ValueType::Schema("list".into()))), Box::new(Remove), vec!["list", "number"]);
    register_simple(registry, info("list.range", "Range", "Builds an arithmetic sequence list", vec![number_channel("start"), ChannelSpec::number_default("step", 1.0), ChannelSpec::number_default("count", 1.0)], ChannelSpec::new("out", ValueType::Schema("list".into()))), Box::new(Range), vec!["number", "number", "number"]);
    register_simple(registry, info("list.reverse", "Reverse", "Reverses indexed list elements", vec![list_channel("list")], ChannelSpec::new("out", ValueType::Schema("list".into()))), Box::new(Reverse), vec!["list"]);
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
        let input = Dictionary::new().insert("list", Value::Dictionary(sample_list())).insert("index", Value::Dictionary(number_dictionary(1.0))).insert("wrap", Value::Dictionary(Dictionary::with_schema("boolean").insert("value", Value::Atom(Atom::Boolean(false)))));
        let out = reg.dispatch("list.get", &input).unwrap();
        assert_eq!(out.schema(), Some("number"));
        assert_eq!(out.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(2.0));
    }

    #[test]
    fn append_adds_next_index() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new().insert("list", Value::Dictionary(sample_list())).insert("value", Value::Dictionary(number_dictionary(4.0)));
        let out = reg.dispatch("list.append", &input).unwrap();
        assert_eq!(out.get("3").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(4.0));
    }

    #[test]
    fn range_builds_sequence() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new().insert("start", Value::Dictionary(number_dictionary(0.0))).insert("step", Value::Dictionary(number_dictionary(2.0))).insert("count", Value::Dictionary(number_dictionary(3.0)));
        let out = reg.dispatch("list.range", &input).unwrap();
        assert_eq!(out.get("1").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(2.0));
    }

    #[test]
    fn manifest_lists_list_operators() {
        let json = build_manifest_json(
            "list",
            "List",
            "0.2.0",
            &module_registry(),
            vec!["onStartup".into()],
            vec![],
            vec![FlowModuleCommandV1 { id: "list.showHelp".into(), title: "List: Show Help".into() }],
            vec![],
        );
        assert!(json.contains("list.append"));
        assert!(json.contains("list.range"));
    }

    #[test]
    fn evaluate_json_size() {
        let input = Dictionary::new().insert("list", Value::Dictionary(sample_list()));
        let out_json = evaluate_json(&module_registry(), "list.size", &serde_json::to_string(&input).unwrap());
        let out: Dictionary = serde_json::from_str(&out_json).unwrap();
        assert_eq!(out.schema(), Some("number"));
        assert_eq!(out.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(3.0));
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
            "0.2.0",
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
