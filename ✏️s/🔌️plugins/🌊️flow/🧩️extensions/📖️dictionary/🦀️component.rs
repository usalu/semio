//! 📚️ Flow dictionary module: operators for dictionary manipulation.

use neural_engine::{channel_output, Atom, ChannelSpec, Dictionary, EvalError, Operator, OperatorImpl, OperatorInfo, Registry, Value, VariadicSpec};

// #region 🔖️Pack
/// 📦️ Wraps input into a dictionary schema.
pub struct Pack;

impl Operator for Pack {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("dictionary", neural_engine::ColdOwner::new(Dictionary::with_schema("dictionary")).merge(input)))
    }
}
// #endregion 🔖️Pack

// #region 🔖️Unpack
/// 📤️ Forwards a dictionary.
pub struct Unpack;

impl Operator for Unpack {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("dictionary", read_dict(input, "dictionary")?.clone()))
    }
}
// #endregion 🔖️Unpack

// #region 🔖️Get
/// 🔍️ Reads a value from a dictionary by key.
pub struct Get;

impl Operator for Get {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let dict = read_dict(input, "dictionary")?;
        let key = read_channel_text(input, "key")?;
        match dict.get(&key).cloned().ok_or(EvalError::MissingInput(key))? {
            Value::Dictionary(value) => Ok(channel_output("value", value)),
            value => Ok(channel_output("value", Dictionary::with_schema("dictionary").insert("value", value))),
        }
    }
}
// #endregion 🔖️Get

// #region 🔖️Set
/// ✏️ Inserts or replaces a key in a dictionary.
pub struct Set;

impl Operator for Set {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let dict = read_dict(input, "dictionary")?;
        let key = read_channel_text(input, "key")?;
        let value = input.get("value").cloned().ok_or_else(|| EvalError::MissingInput("value".into()))?;
        Ok(channel_output("dictionary", dict.clone().insert(key, value)))
    }
}
// #endregion 🔖️Set

// #region 🔖️Remove
/// 🗑️ Removes a key from a dictionary.
pub struct Remove;

impl Operator for Remove {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let dict = read_dict(input, "dictionary")?;
        let key = read_channel_text(input, "key")?;
        Ok(channel_output("dictionary", remove_key(dict, &key)))
    }
}
// #endregion 🔖️Remove

// #region 🔖️Has
/// ❓️ Reports whether a key exists in a dictionary.
pub struct Has;

impl Operator for Has {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let dict = read_dict(input, "dictionary")?;
        let key = read_channel_text(input, "key")?;
        Ok(channel_output("exists", boolean_dictionary(dict.get(&key).is_some())))
    }
}
// #endregion 🔖️Has

// #region 🔖️Keys
/// 🔑️ Lists dictionary keys as comma-separated text.
pub struct Keys;

impl Operator for Keys {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("keys", text_dictionary(read_dict(input, "dictionary")?.keys().map(String::as_str).filter(|key| *key != "$schema").collect::<Vec<_>>().join(","))))
    }
}
// #endregion 🔖️Keys

// #region 🔖️Size
/// 📏️ Reports the number of keys in a dictionary.
pub struct Size;

impl Operator for Size {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let count = read_dict(input, "dictionary")?.keys().filter(|key| key.as_str() != "$schema").count();
        Ok(channel_output("count", number_dictionary(count as f64)))
    }
}
// #endregion 🔖️Size

// #region 🔖️Merge
/// 🔀️ Merges ordered dictionary inputs; later keys override earlier ones.
pub struct Merge;

impl Operator for Merge {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let items = read_dict(input, "items")?;
        let mut indices: Vec<usize> = items.keys().filter_map(|key| key.parse::<usize>().ok()).collect();
        indices.sort_unstable();
        if indices.len() < 2 {
            return Err(EvalError::MissingInput("items".into()));
        }
        let mut merged = neural_engine::ColdDictionaryBuilder::from_dictionary(Dictionary::with_schema("dictionary"));
        for index in indices {
            for (key, value) in read_dict(items, &index.to_string())?.iter() {
                merged.insert(key.clone(), value.clone());
            }
        }
        Ok(channel_output("dictionary", merged.finish()))
    }
}
// #endregion 🔖️Merge

// #region 🔖️Helpers
fn read_dict<'a>(input: &'a Dictionary, key: &str) -> Result<&'a Dictionary, EvalError> {
    input.get(key).and_then(|v| v.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_channel_text(input: &Dictionary, key: &str) -> Result<String, EvalError> {
    input.get(key).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_str()).map(str::to_string).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn text_dictionary(value: String) -> Dictionary {
    Dictionary::with_schema("text").insert("value", Value::Atom(Atom::String(value)))
}

fn number_dictionary(value: f64) -> Dictionary {
    Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(value)))
}

fn boolean_dictionary(value: bool) -> Dictionary {
    Dictionary::with_schema("boolean").insert("value", Value::Atom(Atom::Boolean(value)))
}

fn dict_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::dictionary(id, &[operator_id])
}

fn text_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::text_default(id, "", &[operator_id])
}

fn remove_key(dict: &Dictionary, key: &str) -> Dictionary {
    let mut out = Dictionary::with_schema(dict.schema().unwrap_or("dictionary"));
    for k in dict.keys() {
        if k.as_str() == key || k.as_str() == "$schema" {
            continue;
        }
        if let Some(v) = dict.get(k) {
            out = out.insert(k.clone(), v.clone());
        }
    }
    out
}

fn info(id: &str, name: &str, summary: &str, inputs: Vec<ChannelSpec>, output: ChannelSpec) -> OperatorInfo {
    OperatorInfo { id: id.into(), extension: "dictionary".into(), name: name.into(), abbreviation: name.into(), icon: "emoji:📚️".into(), summary: summary.into(), inputs, outputs: vec![output], ..Default::default() }
}

fn register_simple<O: Operator + 'static>(registry: &mut Registry, info: OperatorInfo, operation: O, schemas: Vec<&str>, produces: &[&str]) {
    registry.register_operator(info, vec![OperatorImpl { schemas: schemas.into_iter().map(str::to_string).collect(), operator: Box::new(operation) }], produces);
}

// #endregion 🔖️Helpers

/// 📦️ Registers all dictionary operators.
pub fn register(registry: &mut Registry) {
    register_simple(registry, info("dictionary.pack", "Pack", "Wraps input as a dictionary", vec![ChannelSpec::wildcard()], ChannelSpec::named("D", "Dic", "dictionary", "PackedDictionary")), Pack, vec![], &["dictionary"]);
    register_simple(
        registry,
        info("dictionary.unpack", "Unpack", "Forwards a dictionary", vec![dict_channel("dictionary", "dictionary.unpack")], ChannelSpec::named("D", "Dic", "dictionary", "UnpackedDictionary")),
        Unpack,
        vec!["dictionary"],
        &["dictionary"],
    );
    register_simple(
        registry,
        info("dictionary.get", "Get", "Reads a value by key", vec![dict_channel("dictionary", "dictionary.get"), text_channel("key", "dictionary.get")], ChannelSpec::named("V", "Val", "value", "DictionaryValue")),
        Get,
        vec!["dictionary", "text"],
        &["value"],
    );
    register_simple(
        registry,
        info(
            "dictionary.set",
            "Set",
            "Inserts or replaces a key",
            vec![dict_channel("dictionary", "dictionary.set"), text_channel("key", "dictionary.set"), ChannelSpec::any("value")],
            ChannelSpec::named("D", "Dic", "dictionary", "UpdatedDictionary"),
        ),
        Set,
        vec![],
        &["dictionary"],
    );
    register_simple(
        registry,
        info("dictionary.remove", "Remove", "Removes a key", vec![dict_channel("dictionary", "dictionary.remove"), text_channel("key", "dictionary.remove")], ChannelSpec::named("D", "Dic", "dictionary", "ReducedDictionary")),
        Remove,
        vec!["dictionary", "text"],
        &["dictionary"],
    );
    register_simple(
        registry,
        info("dictionary.has", "Has", "Checks whether a key exists", vec![dict_channel("dictionary", "dictionary.has"), text_channel("key", "dictionary.has")], ChannelSpec::named("E", "Exs", "exists", "KeyExists")),
        Has,
        vec!["dictionary", "text"],
        &["boolean"],
    );
    register_simple(registry, info("dictionary.keys", "Keys", "Lists keys as comma-separated text", vec![dict_channel("dictionary", "dictionary.keys")], ChannelSpec::named("K", "Key", "keys", "DictionaryKeys")), Keys, vec!["dictionary"], &["text"]);
    register_simple(registry, info("dictionary.size", "Size", "Reports the number of keys", vec![dict_channel("dictionary", "dictionary.size")], ChannelSpec::named("C", "Cnt", "count", "DictionaryCount")), Size, vec!["dictionary"], &["number"]);
    registry.register_operator(
        OperatorInfo {
            variadic_input: Some(VariadicSpec { slot_key: "items".into(), min: 2, max: None }),
            ..info("dictionary.merge", "Merge", "Merges ordered dictionary inputs", vec![], ChannelSpec::named("D", "Dic", "dictionary", "MergedDictionary"))
        },
        vec![OperatorImpl { schemas: vec!["dictionary".into(), "dictionary".into()], operator: Box::new(Merge) }],
        &["dictionary"],
    );
    registry.finalize();
}

// #region 🔖️Manifest
/// 📦️ Flow extension manifest JSON contributed to host catalogues.
pub fn extension_manifest_json() -> String {
    use flow_extension_sdk::{build_manifest_json, FlowExtensionCommand};
    build_manifest_json("dictionary", "Dictionary", "0.1.0", &neural_engine::ColdOwner::new(module_registry()), vec!["onStartup".into()], vec![], vec![FlowExtensionCommand { id: "dictionary.showHelp".into(), title: "Dictionary: Show Help".into() }], vec![])
}

/// 🌊️ Builds an in-process operator registry for this extension.
pub fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
}
// #endregion 🔖️Manifest

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use flow_extension_sdk::{build_manifest_json, evaluate_json, FlowExtensionCommand};

    fn sample_dict() -> Dictionary {
        Dictionary::with_schema("dictionary").insert("number", Value::Dictionary(number_dictionary(3.0))).insert("text", Value::Dictionary(text_dictionary("hi".into())))
    }

    #[semio_framework_async_macros::async_test]
    async fn get_reads_value() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new().insert("dictionary", Value::Dictionary(sample_dict())).insert("key", Value::Dictionary(text_dictionary("number".into())));
        let out = reg.dispatch("dictionary.get", &input).unwrap();
        let value = out.get("value").and_then(|v| v.as_dictionary()).expect("value channel");
        assert_eq!(value.schema(), Some("number"));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_inserts_key() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new().insert("dictionary", Value::Dictionary(Dictionary::with_schema("dictionary"))).insert("key", Value::Dictionary(text_dictionary("text".into()))).insert("value", Value::Dictionary(text_dictionary("new".into())));
        let out = reg.dispatch("dictionary.set", &input).unwrap();
        let dictionary = out.get("dictionary").and_then(|v| v.as_dictionary()).expect("dictionary channel");
        assert!(dictionary.get("text").is_some());
    }

    #[semio_framework_async_macros::async_test]
    async fn merge_combines_dicts() {
        let mut reg = Registry::new();
        register(&mut reg);
        let items = Dictionary::new()
            .insert("0", Value::Dictionary(Dictionary::with_schema("dictionary").insert("a", Value::Dictionary(number_dictionary(1.0)))))
            .insert("1", Value::Dictionary(Dictionary::with_schema("dictionary").insert("b", Value::Dictionary(text_dictionary("x".into())))));
        let out = reg.dispatch("dictionary.merge", &Dictionary::new().insert("items", Value::Dictionary(items))).unwrap();
        let dictionary = out.get("dictionary").and_then(|v| v.as_dictionary()).expect("dictionary channel");
        assert_eq!(dictionary.schema(), Some("dictionary"));
        assert!(dictionary.get("a").is_some());
        assert!(dictionary.get("b").is_some());
    }

    #[semio_framework_async_macros::async_test]
    async fn manifest_lists_dictionary_operators() {
        let json = build_manifest_json("dictionary", "Dictionary", "0.1.0", &neural_engine::ColdOwner::new(module_registry()), vec!["onStartup".into()], vec![], vec![FlowExtensionCommand { id: "dictionary.showHelp".into(), title: "Dictionary: Show Help".into() }], vec![]);
        assert!(json.contains("dictionary.get"));
    }

    #[semio_framework_async_macros::async_test]
    async fn evaluate_json_pack() {
        let out_json = evaluate_json(&neural_engine::ColdOwner::new(module_registry()), "dictionary.pack", &serde_json::to_string(&sample_dict()).unwrap());
        let out: Dictionary = serde_json::from_str(&out_json).unwrap();
        let dictionary = out.get("dictionary").and_then(|v| v.as_dictionary()).expect("dictionary channel");
        assert_eq!(dictionary.schema(), Some("dictionary"));
    }
}
// #endregion 🔖️Tests

// #region 🔖️ExtensionGuest
/// 🧩️ Runtime-installable flow extension bundle for `dictionary`.
#[cfg(feature = "component-guest")]
mod extension_guest {
    use super::{extension_manifest_json, module_registry};
    use flow_extension_sdk::evaluate_json;
    use semio_framework::{Fault, FaultCode, FaultOrigin};
    use semio_framework_plugin::{ExecutionMode, ExtensionBundle};
    use serde::Deserialize;

    const FLOW_APP_ID: &str = "flow-play";
    const PROCEDURAL3D_APP_ID: &str = "procedural3d-play";
    const EXTENSION_ID: &str = "dictionary";
    const EXTENSION_LABEL: &str = "Dictionary";

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct EvaluateRequest {
        operator_id: String,
        input_json: String,
    }

    fn flow_extension_contribution(app_id: &str, manifest_json: String) -> serde_json::Value {
        let icon_id = "dictionary";
        let topic_payload = serde_json::json!({
            "appId": app_id,
            "extensionId": EXTENSION_ID,
            "label": EXTENSION_LABEL,
            "iconId": icon_id,
            "manifestJson": &manifest_json,
        });
        topic_payload
    }

    // 🚫️async: E1 pure — `extension_exports!` calls `bundle` outside an async context (macro requires
    // a plain sync fn). `.mode`/`.contributes_topic`/`.handler` are still `async fn` in
    // `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (out of this packet's
    // path_scope); bridged via `semio_framework::io::resolve_ready` — see this packet's lease-request.
    // See R9.
    fn bundle() -> ExtensionBundle {
        let manifest_json = extension_manifest_json();
        let flow_topic_payload = flow_extension_contribution(FLOW_APP_ID, manifest_json.clone());
        let procedural3d_topic_payload = flow_extension_contribution(PROCEDURAL3D_APP_ID, manifest_json);
        let bundle = ExtensionBundle::new(EXTENSION_ID, EXTENSION_LABEL, "0.1.0").extends("flow");
        let bundle = semio_framework::io::resolve_ready(bundle.mode(ExecutionMode::Linked));
        let bundle = semio_framework::io::resolve_ready(bundle.contributes_topic("flow.extension", flow_topic_payload));
        let bundle = semio_framework::io::resolve_ready(bundle.contributes_topic("flow.extension", procedural3d_topic_payload));
        semio_framework::io::resolve_ready(bundle.handler("evaluate", |req| {
            let request: EvaluateRequest = serde_json::from_slice(req).map_err(|err| Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.evaluate.bad-request"), err.to_string()))?;
            Ok(evaluate_json(&neural_engine::ColdOwner::new(module_registry()), &request.operator_id, &request.input_json).into_bytes())
        }))
    }

    semio_framework_plugin::extension_exports!(bundle);
}
// #endregion 🔖️ExtensionGuest
