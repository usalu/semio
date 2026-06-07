//! 📚 Flow dictionary module: neuron kinds for dictionary manipulation.

use neural_engine::{Atom, Dictionary, EvalError, Function, NeuronKindInfo, Registry, Value, VariadicSpec};

// #region 🔖Pack
/// 📦 Wraps the entire input dictionary under a single dictionary value.
pub struct Pack;

impl Function for Pack {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(Dictionary::new().insert("dictionary", Value::Dictionary(input.clone())))
    }
}
// #endregion 🔖Pack

// #region 🔖Unpack
/// 📤 Flattens a nested dictionary value to top-level keys.
pub struct Unpack;

impl Function for Unpack {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let dict = read_dict(input, "dictionary")?;
        Ok(flatten_dict(dict))
    }
}
// #endregion 🔖Unpack

// #region 🔖Get
/// 🔍 Reads a value from a dictionary by key.
pub struct Get;

impl Function for Get {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let dict = read_dict(input, "dictionary")?;
        let key = read_text(input, "key")?;
        let value = dict.get(&key).cloned().ok_or_else(|| EvalError::MissingInput(key))?;
        Ok(Dictionary::new().insert("value", value))
    }
}
// #endregion 🔖Get

// #region 🔖Set
/// ✏️ Inserts or replaces a key in a dictionary.
pub struct Set;

impl Function for Set {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let dict = read_dict(input, "dictionary")?;
        let key = read_text(input, "key")?;
        let value = read_value(input, "value")?;
        Ok(Dictionary::new().insert("dictionary", Value::Dictionary(dict.clone().insert(key, value))))
    }
}
// #endregion 🔖Set

// #region 🔖Remove
/// 🗑️ Removes a key from a dictionary.
pub struct Remove;

impl Function for Remove {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let dict = read_dict(input, "dictionary")?;
        let key = read_text(input, "key")?;
        Ok(Dictionary::new().insert("dictionary", Value::Dictionary(remove_key(dict, &key))))
    }
}
// #endregion 🔖Remove

// #region 🔖Has
/// ❓ Reports whether a key exists in a dictionary.
pub struct Has;

impl Function for Has {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let dict = read_dict(input, "dictionary")?;
        let key = read_text(input, "key")?;
        let flag = if dict.get(&key).is_some() { 1.0 } else { 0.0 };
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(flag))))
    }
}
// #endregion 🔖Has

// #region 🔖Keys
/// 🔑 Lists dictionary keys as comma-separated text.
pub struct Keys;

impl Function for Keys {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let dict = read_dict(input, "dictionary")?;
        let text = dict.keys().map(String::as_str).collect::<Vec<_>>().join(",");
        Ok(Dictionary::new().insert("text", Value::Atom(Atom::String(text))))
    }
}
// #endregion 🔖Keys

// #region 🔖Size
/// 📏 Reports the number of keys in a dictionary.
pub struct Size;

impl Function for Size {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let dict = read_dict(input, "dictionary")?;
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(dict.len() as f64))))
    }
}
// #endregion 🔖Size

// #region 🔖Merge
/// 🔀 Merges ordered dictionary inputs; later keys override earlier ones.
pub struct Merge;

impl Function for Merge {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let items = read_dict(input, "items")?;
        let mut indices: Vec<usize> = items.keys().filter_map(|key| key.parse::<usize>().ok()).collect();
        indices.sort_unstable();
        if indices.len() < 2 {
            return Err(EvalError::MissingInput("items".into()));
        }
        let mut merged = Dictionary::new();
        for index in indices {
            let slot = read_dict(items, &index.to_string())?;
            merged = merged.merge(slot);
        }
        Ok(Dictionary::new().insert("dictionary", Value::Dictionary(merged)))
    }
}
// #endregion 🔖Merge

fn read_dict<'a>(input: &'a Dictionary, key: &str) -> Result<&'a Dictionary, EvalError> {
    input
        .get(key)
        .and_then(|v| v.as_dictionary())
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

fn read_value(input: &Dictionary, key: &str) -> Result<Value, EvalError> {
    input.get(key).cloned().ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn flatten_dict(dict: &Dictionary) -> Dictionary {
    let mut out = Dictionary::new();
    for k in dict.keys() {
        if let Some(v) = dict.get(k) {
            out = out.insert(k.clone(), v.clone());
        }
    }
    out
}

fn remove_key(dict: &Dictionary, key: &str) -> Dictionary {
    let mut out = Dictionary::new();
    for k in dict.keys() {
        if k.as_str() != key {
            if let Some(v) = dict.get(k) {
                out = out.insert(k.clone(), v.clone());
            }
        }
    }
    out
}

fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
}

/// 📦 Registers all dictionary neuron kinds on the registry.
pub fn register(registry: &mut Registry) {
    registry.register(
        NeuronKindInfo {
            id: "dictionary.pack".into(),
            module: "dictionary".into(),
            name: "Pack".into(),
            abbreviation: "Pack".into(),
            icon: "emoji:📦".into(),
            summary: "Wraps input as a nested dictionary".into(),
            inputs: vec!["*".into()],
            outputs: vec!["dictionary".into()],
            ..Default::default()
        },
        Box::new(Pack),
    );
    registry.register(
        NeuronKindInfo {
            id: "dictionary.unpack".into(),
            module: "dictionary".into(),
            name: "Unpack".into(),
            abbreviation: "Unpack".into(),
            icon: "emoji:📤".into(),
            summary: "Flattens a nested dictionary to top-level keys".into(),
            inputs: vec!["dictionary".into()],
            outputs: vec!["*".into()],
            ..Default::default()
        },
        Box::new(Unpack),
    );
    registry.register(
        NeuronKindInfo {
            id: "dictionary.get".into(),
            module: "dictionary".into(),
            name: "Get".into(),
            abbreviation: "Get".into(),
            icon: "emoji:🔍".into(),
            summary: "Reads a value by key".into(),
            inputs: vec!["dictionary".into(), "key".into()],
            outputs: vec!["value".into()],
            ..Default::default()
        },
        Box::new(Get),
    );
    registry.register(
        NeuronKindInfo {
            id: "dictionary.set".into(),
            module: "dictionary".into(),
            name: "Set".into(),
            abbreviation: "Set".into(),
            icon: "emoji:✏️".into(),
            summary: "Inserts or replaces a key".into(),
            inputs: vec!["dictionary".into(), "key".into(), "value".into()],
            outputs: vec!["dictionary".into()],
            ..Default::default()
        },
        Box::new(Set),
    );
    registry.register(
        NeuronKindInfo {
            id: "dictionary.remove".into(),
            module: "dictionary".into(),
            name: "Remove".into(),
            abbreviation: "Remove".into(),
            icon: "emoji:🗑️".into(),
            summary: "Removes a key".into(),
            inputs: vec!["dictionary".into(), "key".into()],
            outputs: vec!["dictionary".into()],
            ..Default::default()
        },
        Box::new(Remove),
    );
    registry.register(
        NeuronKindInfo {
            id: "dictionary.has".into(),
            module: "dictionary".into(),
            name: "Has".into(),
            abbreviation: "Has".into(),
            icon: "emoji:❓".into(),
            summary: "Checks whether a key exists".into(),
            inputs: vec!["dictionary".into(), "key".into()],
            outputs: vec!["number".into()],
            ..Default::default()
        },
        Box::new(Has),
    );
    registry.register(
        NeuronKindInfo {
            id: "dictionary.keys".into(),
            module: "dictionary".into(),
            name: "Keys".into(),
            abbreviation: "Keys".into(),
            icon: "emoji:🔑".into(),
            summary: "Lists keys as comma-separated text".into(),
            inputs: vec!["dictionary".into()],
            outputs: vec!["text".into()],
            ..Default::default()
        },
        Box::new(Keys),
    );
    registry.register(
        NeuronKindInfo {
            id: "dictionary.size".into(),
            module: "dictionary".into(),
            name: "Size".into(),
            abbreviation: "Size".into(),
            icon: "emoji:📏".into(),
            summary: "Reports the number of keys".into(),
            inputs: vec!["dictionary".into()],
            outputs: vec!["number".into()],
            ..Default::default()
        },
        Box::new(Size),
    );
    registry.register(
        NeuronKindInfo {
            id: "dictionary.merge".into(),
            module: "dictionary".into(),
            name: "Merge".into(),
            abbreviation: "Merge".into(),
            icon: "emoji:🔀".into(),
            summary: "Merges ordered dictionary inputs".into(),
            inputs: vec![],
            outputs: vec!["dictionary".into()],
            variadic_input: Some(VariadicSpec { slot_key: "items".into(), min: 2, max: None }),
            variadic_output: None,
        },
        Box::new(Merge),
    );
}

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_dict() -> Dictionary {
        Dictionary::new()
            .insert("number", Value::Atom(Atom::Decimal(3.0)))
            .insert("text", Value::Atom(Atom::String("hi".into())))
    }

    #[test]
    fn pack_wraps_input() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = sample_dict();
        let out = reg.get("dictionary.pack").unwrap().evaluate(&input).unwrap();
        let nested = out.get("dictionary").and_then(|v| v.as_dictionary()).expect("nested");
        assert_eq!(nested.get("number").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(3.0));
    }

    #[test]
    fn unpack_flattens_dictionary() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new().insert("dictionary", Value::Dictionary(sample_dict()));
        let out = reg.get("dictionary.unpack").unwrap().evaluate(&input).unwrap();
        assert_eq!(out.get("text").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("hi"));
    }

    #[test]
    fn get_reads_value() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new()
            .insert("dictionary", Value::Dictionary(sample_dict()))
            .insert("key", Value::Atom(Atom::String("number".into())));
        let out = reg.get("dictionary.get").unwrap().evaluate(&input).unwrap();
        assert_eq!(out.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(3.0));
    }

    #[test]
    fn set_inserts_key() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new()
            .insert("dictionary", Value::Dictionary(Dictionary::new()))
            .insert("key", Value::Atom(Atom::String("text".into())))
            .insert("value", Value::Atom(Atom::String("new".into())));
        let out = reg.get("dictionary.set").unwrap().evaluate(&input).unwrap();
        let dict = out.get("dictionary").and_then(|v| v.as_dictionary()).expect("dict");
        assert_eq!(dict.get("text").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("new"));
    }

    #[test]
    fn remove_drops_key() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new()
            .insert("dictionary", Value::Dictionary(sample_dict()))
            .insert("key", Value::Atom(Atom::String("text".into())));
        let out = reg.get("dictionary.remove").unwrap().evaluate(&input).unwrap();
        let dict = out.get("dictionary").and_then(|v| v.as_dictionary()).expect("dict");
        assert!(dict.get("text").is_none());
        assert_eq!(dict.get("number").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(3.0));
    }

    #[test]
    fn has_reports_presence() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new()
            .insert("dictionary", Value::Dictionary(sample_dict()))
            .insert("key", Value::Atom(Atom::String("text".into())));
        let out = reg.get("dictionary.has").unwrap().evaluate(&input).unwrap();
        assert_eq!(out.get("number").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(1.0));
    }

    #[test]
    fn keys_lists_keys() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new().insert("dictionary", Value::Dictionary(sample_dict()));
        let out = reg.get("dictionary.keys").unwrap().evaluate(&input).unwrap();
        let text = out.get("text").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).expect("text");
        assert!(text.contains("number"));
        assert!(text.contains("text"));
    }

    #[test]
    fn size_reports_len() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new().insert("dictionary", Value::Dictionary(sample_dict()));
        let out = reg.get("dictionary.size").unwrap().evaluate(&input).unwrap();
        assert_eq!(out.get("number").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(2.0));
    }

    #[test]
    fn merge_combines_dicts() {
        let mut reg = Registry::new();
        register(&mut reg);
        let a = Dictionary::new().insert("number", Value::Atom(Atom::Decimal(1.0)));
        let b = Dictionary::new().insert("text", Value::Atom(Atom::String("x".into())));
        let items = Dictionary::new()
            .insert("0", Value::Dictionary(a))
            .insert("1", Value::Dictionary(b));
        let input = Dictionary::new().insert("items", Value::Dictionary(items));
        let out = reg.get("dictionary.merge").unwrap().evaluate(&input).unwrap();
        let dict = out.get("dictionary").and_then(|v| v.as_dictionary()).expect("dict");
        assert_eq!(dict.get("number").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(1.0));
        assert_eq!(dict.get("text").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("x"));
    }

    #[test]
    fn merge_three_way_later_overrides() {
        let mut reg = Registry::new();
        register(&mut reg);
        let first = Dictionary::new().insert("number", Value::Atom(Atom::Decimal(1.0)));
        let second = Dictionary::new().insert("number", Value::Atom(Atom::Decimal(2.0)));
        let third = Dictionary::new().insert("text", Value::Atom(Atom::String("z".into())));
        let items = Dictionary::new()
            .insert("0", Value::Dictionary(first))
            .insert("1", Value::Dictionary(second))
            .insert("2", Value::Dictionary(third));
        let input = Dictionary::new().insert("items", Value::Dictionary(items));
        let out = reg.get("dictionary.merge").unwrap().evaluate(&input).unwrap();
        let dict = out.get("dictionary").and_then(|v| v.as_dictionary()).expect("dict");
        assert_eq!(dict.get("number").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(2.0));
        assert_eq!(dict.get("text").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("z"));
    }

    use flow_module_wasm::{build_manifest_json, evaluate_json, FlowModuleCommandV1};

    #[test]
    fn manifest_lists_dictionary_kinds() {
        let json = build_manifest_json(
            "dictionary",
            "Dictionary",
            "0.1.0",
            &module_registry(),
            vec!["onStartup".into()],
            vec![],
            vec![FlowModuleCommandV1 { id: "dictionary.showHelp".into(), title: "Dictionary: Show Help".into() }],
            vec![],
        );
        assert!(json.contains("dictionary.get"));
    }

    #[test]
    fn evaluate_json_pack() {
        let reg = module_registry();
        let input = sample_dict();
        let out_json = evaluate_json(&reg, "dictionary.pack", &serde_json::to_string(&input).unwrap());
        let out: Dictionary = serde_json::from_str(&out_json).unwrap();
        let nested = out.get("dictionary").and_then(|v| v.as_dictionary()).expect("nested");
        assert_eq!(nested.get("number").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(3.0));
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
            "dictionary",
            "Dictionary",
            "0.1.0",
            &module_registry(),
            vec!["onStartup".into()],
            vec![],
            vec![FlowModuleCommandV1 { id: "dictionary.showHelp".into(), title: "Dictionary: Show Help".into() }],
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
