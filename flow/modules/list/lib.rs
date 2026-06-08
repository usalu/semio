//! 📋 Flow list module: index-keyed dictionaries as lists.

use neural_engine::{Atom, Dictionary, EvalError, Function, InputSpec, NeuronKindInfo, Registry, Value};

// #region 🔖Empty
/// 🆕 Creates an empty list dictionary.
pub struct Empty;

impl Function for Empty {
    fn evaluate(&self, _input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(Dictionary::new().insert("list", Value::Dictionary(Dictionary::new())))
    }
}
// #endregion 🔖Empty

// #region 🔖Pack
/// 📦 Wraps the entire input dictionary as a list value.
pub struct Pack;

impl Function for Pack {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(Dictionary::new().insert("list", Value::Dictionary(input.clone())))
    }
}
// #endregion 🔖Pack

// #region 🔖Get
/// 🔍 Reads a list element by index.
pub struct Get;

impl Function for Get {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let list = read_list(input, "list")?;
        let len = list_len(list);
        let wrap = read_bool(input, "wrap");
        let index = resolve_list_index(input, "index", len, wrap)?;
        let key = index.to_string();
        let value = list.get(&key).cloned().ok_or_else(|| EvalError::MissingInput(key))?;
        Ok(Dictionary::new().insert("value", value))
    }
}
// #endregion 🔖Get

// #region 🔖Set
/// ✏️ Replaces a list element at an index.
pub struct Set;

impl Function for Set {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let list = read_list(input, "list")?;
        let index = read_index(input, "index")?;
        let value = read_value(input, "value")?;
        Ok(Dictionary::new().insert("list", Value::Dictionary(list.clone().insert(index.to_string(), value))))
    }
}
// #endregion 🔖Set

// #region 🔖Append
/// ➕ Appends a value at the next list index.
pub struct Append;

impl Function for Append {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let list = read_list(input, "list")?;
        let value = read_value(input, "value")?;
        let next = next_list_index(list);
        Ok(Dictionary::new().insert("list", Value::Dictionary(list.clone().insert(next.to_string(), value))))
    }
}
// #endregion 🔖Append

// #region 🔖Size
/// 📏 Reports the number of indexed list elements.
pub struct Size;

impl Function for Size {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let list = read_list(input, "list")?;
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(list_len(list) as f64))))
    }
}
// #endregion 🔖Size

// #region 🔖Remove
/// 🗑️ Removes a list element and reindexes remaining items.
pub struct Remove;

impl Function for Remove {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let list = read_list(input, "list")?;
        let index = read_index(input, "index")?;
        Ok(Dictionary::new().insert("list", Value::Dictionary(remove_list_index(list, index))))
    }
}
// #endregion 🔖Remove

// #region 🔖Range
/// 📈 Builds an arithmetic sequence list.
pub struct Range;

impl Function for Range {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let start = read_number(input, "start").unwrap_or(0.0);
        let step = read_number(input, "step").unwrap_or(1.0);
        let count = read_index(input, "count")?;
        let mut list = Dictionary::new();
        for index in 0..count {
            list = list.insert(index.to_string(), Value::Atom(Atom::Decimal(start + step * index as f64)));
        }
        Ok(Dictionary::new().insert("list", Value::Dictionary(list)))
    }
}
// #endregion 🔖Range

// #region 🔖Reverse
/// 🔁 Reverses indexed list elements.
pub struct Reverse;

impl Function for Reverse {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let list = read_list(input, "list")?;
        let indices = list_indices(list);
        let mut out = Dictionary::new();
        for (next, index) in indices.iter().rev().enumerate() {
            if let Some(value) = list.get(&index.to_string()) {
                out = out.insert(next.to_string(), value.clone());
            }
        }
        Ok(Dictionary::new().insert("list", Value::Dictionary(out)))
    }
}
// #endregion 🔖Reverse

fn read_number(input: &Dictionary, key: &str) -> Option<f64> {
    input.get(key).and_then(|v| v.as_atom()).and_then(|a| a.as_f64())
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

fn read_list<'a>(input: &'a Dictionary, key: &str) -> Result<&'a Dictionary, EvalError> {
    input
        .get(key)
        .and_then(|v| v.as_dictionary())
        .ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_bool(input: &Dictionary, key: &str) -> bool {
    input.get(key).and_then(|v| v.as_atom()).and_then(|a| a.as_bool()).unwrap_or(false)
}

fn read_index(input: &Dictionary, key: &str) -> Result<usize, EvalError> {
    let raw = input
        .get(key)
        .and_then(|v| v.as_atom())
        .and_then(|a| a.as_f64())
        .ok_or_else(|| EvalError::MissingInput(key.into()))?;
    Ok(raw as usize)
}

fn resolve_list_index(input: &Dictionary, key: &str, len: usize, wrap: bool) -> Result<usize, EvalError> {
    let raw = input
        .get(key)
        .and_then(|v| v.as_atom())
        .and_then(|a| a.as_f64())
        .ok_or_else(|| EvalError::MissingInput(key.into()))?;
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

fn read_value(input: &Dictionary, key: &str) -> Result<Value, EvalError> {
    input.get(key).cloned().ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn remove_list_index(list: &Dictionary, remove_at: usize) -> Dictionary {
    let indices = list_indices(list);
    let mut out = Dictionary::new();
    let mut next = 0usize;
    for index in indices {
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

fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
}

/// 📦 Registers all list neuron kinds on the registry.
pub fn register(registry: &mut Registry) {
    registry.register(
        NeuronKindInfo {
            id: "list.empty".into(),
            module: "list".into(),
            name: "Empty".into(),
            abbreviation: "Empty".into(),
            icon: "emoji:🆕".into(),
            summary: "Creates an empty list".into(),
            inputs: vec![],
            outputs: vec!["list".into()],
            ..Default::default()
        },
        Box::new(Empty),
    );
    registry.register(
        NeuronKindInfo {
            id: "list.pack".into(),
            module: "list".into(),
            name: "Pack".into(),
            abbreviation: "Pack".into(),
            icon: "emoji:📦".into(),
            summary: "Wraps input as a list dictionary".into(),
            inputs: vec![InputSpec::wildcard()],
            outputs: vec!["list".into()],
            ..Default::default()
        },
        Box::new(Pack),
    );
    registry.register(
        NeuronKindInfo {
            id: "list.get".into(),
            module: "list".into(),
            name: "Get".into(),
            abbreviation: "Get".into(),
            icon: "emoji:🔍".into(),
            summary: "Reads a value by index".into(),
            inputs: vec![
                InputSpec::list("list"),
                InputSpec::number_default("index", 0.0),
                InputSpec::boolean_default("wrap", false),
            ],
            outputs: vec!["value".into()],
            ..Default::default()
        },
        Box::new(Get),
    );
    registry.register(
        NeuronKindInfo {
            id: "list.set".into(),
            module: "list".into(),
            name: "Set".into(),
            abbreviation: "Set".into(),
            icon: "emoji:✏️".into(),
            summary: "Replaces a value at an index".into(),
            inputs: vec![InputSpec::list("list"), InputSpec::number_default("index", 0.0), InputSpec::value("value")],
            outputs: vec!["list".into()],
            ..Default::default()
        },
        Box::new(Set),
    );
    registry.register(
        NeuronKindInfo {
            id: "list.append".into(),
            module: "list".into(),
            name: "Append".into(),
            abbreviation: "Append".into(),
            icon: "emoji:➕".into(),
            summary: "Appends a value at the next index".into(),
            inputs: vec![InputSpec::list("list"), InputSpec::value("value")],
            outputs: vec!["list".into()],
            ..Default::default()
        },
        Box::new(Append),
    );
    registry.register(
        NeuronKindInfo {
            id: "list.size".into(),
            module: "list".into(),
            name: "Size".into(),
            abbreviation: "Size".into(),
            icon: "emoji:📏".into(),
            summary: "Reports the number of indexed elements".into(),
            inputs: vec![InputSpec::list("list")],
            outputs: vec!["number".into()],
            ..Default::default()
        },
        Box::new(Size),
    );
    registry.register(
        NeuronKindInfo {
            id: "list.remove".into(),
            module: "list".into(),
            name: "Remove".into(),
            abbreviation: "Remove".into(),
            icon: "emoji:🗑️".into(),
            summary: "Removes an index and reindexes".into(),
            inputs: vec![InputSpec::list("list"), InputSpec::number_default("index", 0.0)],
            outputs: vec!["list".into()],
            ..Default::default()
        },
        Box::new(Remove),
    );
    registry.register(
        NeuronKindInfo {
            id: "list.range".into(),
            module: "list".into(),
            name: "Range".into(),
            abbreviation: "Range".into(),
            icon: "emoji:📈".into(),
            summary: "Builds an arithmetic sequence list".into(),
            inputs: vec![
                InputSpec::number_default("start", 0.0),
                InputSpec::number_default("step", 1.0),
                InputSpec::number_default("count", 1.0),
            ],
            outputs: vec!["list".into()],
            ..Default::default()
        },
        Box::new(Range),
    );
    registry.register(
        NeuronKindInfo {
            id: "list.reverse".into(),
            module: "list".into(),
            name: "Reverse".into(),
            abbreviation: "Rev".into(),
            icon: "emoji:🔁".into(),
            summary: "Reverses indexed list elements".into(),
            inputs: vec![InputSpec::list("list")],
            outputs: vec!["list".into()],
            ..Default::default()
        },
        Box::new(Reverse),
    );
}

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use neural_engine::inject_input_defaults;

    fn sample_list() -> Dictionary {
        Dictionary::new()
            .insert("0", Value::Atom(Atom::Decimal(1.0)))
            .insert("1", Value::Atom(Atom::Decimal(2.0)))
            .insert("2", Value::Atom(Atom::Decimal(3.0)))
    }

    #[test]
    fn empty_creates_list() {
        let mut reg = Registry::new();
        register(&mut reg);
        let out = reg.get("list.empty").unwrap().evaluate(&Dictionary::new()).unwrap();
        assert!(out.get("list").and_then(|v| v.as_dictionary()).is_some());
    }

    #[test]
    fn get_defaults_index_zero() {
        let mut reg = Registry::new();
        register(&mut reg);
        let kind = reg.kind_info("list.get").expect("kind");
        let input = inject_input_defaults(
            Dictionary::new().insert("list", Value::Dictionary(sample_list())),
            kind,
        );
        let out = reg.get("list.get").unwrap().evaluate(&input).unwrap();
        assert_eq!(out.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(1.0));
    }

    #[test]
    fn get_wraps_out_of_range_index() {
        let mut reg = Registry::new();
        register(&mut reg);
        let kind = reg.kind_info("list.get").expect("kind");
        let input = inject_input_defaults(
            Dictionary::new()
                .insert("list", Value::Dictionary(sample_list()))
                .insert("index", Value::Atom(Atom::Decimal(5.0)))
                .insert("wrap", Value::Atom(Atom::Boolean(true))),
            kind,
        );
        let out = reg.get("list.get").unwrap().evaluate(&input).unwrap();
        assert_eq!(out.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(3.0));
    }

    #[test]
    fn get_reads_index() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new()
            .insert("list", Value::Dictionary(sample_list()))
            .insert("index", Value::Atom(Atom::Decimal(1.0)));
        let out = reg.get("list.get").unwrap().evaluate(&input).unwrap();
        assert_eq!(out.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(2.0));
    }

    #[test]
    fn append_adds_next_index() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new()
            .insert("list", Value::Dictionary(sample_list()))
            .insert("value", Value::Atom(Atom::Decimal(4.0)));
        let out = reg.get("list.append").unwrap().evaluate(&input).unwrap();
        let list = out.get("list").and_then(|v| v.as_dictionary()).expect("list");
        assert_eq!(list.get("3").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(4.0));
    }

    #[test]
    fn remove_reindexes() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new()
            .insert("list", Value::Dictionary(sample_list()))
            .insert("index", Value::Atom(Atom::Decimal(1.0)));
        let out = reg.get("list.remove").unwrap().evaluate(&input).unwrap();
        let list = out.get("list").and_then(|v| v.as_dictionary()).expect("list");
        assert_eq!(list_len(list), 2);
        assert_eq!(list.get("0").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(1.0));
        assert_eq!(list.get("1").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(3.0));
    }

    use flow_module_wasm::{build_manifest_json, evaluate_json, FlowModuleCommandV1};

    #[test]
    fn range_builds_sequence() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new()
            .insert("start", Value::Atom(Atom::Decimal(0.0)))
            .insert("step", Value::Atom(Atom::Decimal(2.0)))
            .insert("count", Value::Atom(Atom::Decimal(3.0)));
        let out = reg.get("list.range").unwrap().evaluate(&input).unwrap();
        let list = out.get("list").and_then(|v| v.as_dictionary()).expect("list");
        assert_eq!(list.get("1").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(2.0));
    }

    #[test]
    fn reverse_reorders_list() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new().insert("list", Value::Dictionary(sample_list()));
        let out = reg.get("list.reverse").unwrap().evaluate(&input).unwrap();
        let list = out.get("list").and_then(|v| v.as_dictionary()).expect("list");
        assert_eq!(list.get("0").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(3.0));
        assert_eq!(list.get("2").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(1.0));
    }

    #[test]
    fn manifest_lists_list_kinds() {
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
        let reg = module_registry();
        let input = Dictionary::new().insert("list", Value::Dictionary(sample_list()));
        let out_json = evaluate_json(&reg, "list.size", &serde_json::to_string(&input).unwrap());
        let out: Dictionary = serde_json::from_str(&out_json).unwrap();
        assert_eq!(out.get("number").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(3.0));
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
