//! 📝️ Flow text module: operators for text dictionaries.

use neural_engine::{channel_output, Atom, ChannelSpec, Dictionary, EvalError, Operator, OperatorImpl, OperatorInfo, Registry, Value};

// #region 🔖️Concat
/// 🔗️ Joins two text inputs.
pub struct Concat;

impl Operator for Concat {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("text", text_dictionary(format!("{}{}", read_channel_text(input, "a")?, read_channel_text(input, "b")?))))
    }
}
// #endregion 🔖️Concat

// #region 🔖️Upper
/// 🔠️ Uppercases a text input.
pub struct Upper;

impl Operator for Upper {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("text", text_dictionary(read_channel_text(input, "text")?.to_uppercase())))
    }
}
// #endregion 🔖️Upper

// #region 🔖️Helpers
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
    OperatorInfo { id: id.into(), extension: "text".into(), name: name.into(), abbreviation: name.into(), icon: "emoji:📝️".into(), summary: summary.into(), inputs, outputs: vec![output], ..Default::default() }
}

// #endregion 🔖️Helpers

/// 📦️ Registers all text operators.
pub fn register(registry: &mut Registry) {
    registry.register_operator(
        info("text.concat", "Concat", "Joins two text values", vec![text_channel("a", "text.concat"), text_channel("b", "text.concat")], ChannelSpec::named("T", "Txt", "text", "JoinedText")),
        vec![OperatorImpl { schemas: vec!["text".into(), "text".into()], operator: Box::new(Concat) }],
        &["text"],
    );
    registry.register_operator(
        info("text.upper", "Upper", "Uppercases text", vec![text_channel("text", "text.upper")], ChannelSpec::named("T", "Txt", "text", "UppercasedText")),
        vec![OperatorImpl { schemas: vec!["text".into()], operator: Box::new(Upper) }],
        &["text"],
    );
    registry.finalize();
}

// #region 🔖️Manifest
/// 📦️ Flow extension manifest JSON contributed to host catalogues.
pub fn extension_manifest_json() -> String {
    use flow_extension_sdk::{build_manifest_json, FlowExtensionCommand};
    build_manifest_json("text", "Text", "0.1.0", &neural_engine::ColdOwner::new(module_registry()), vec!["onStartup".into()], vec![], vec![FlowExtensionCommand { id: "text.showHelp".into(), title: "Text: Show Help".into() }], vec![])
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

    #[semio_framework_async_macros::async_test]
    async fn concat_joins_text() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new().insert("a", Value::Dictionary(text_dictionary("hi".into()))).insert("b", Value::Dictionary(text_dictionary("!".into())));
        let out = reg.dispatch("text.concat", &input).unwrap();
        let text = out.get("text").and_then(|v| v.as_dictionary()).expect("text channel");
        assert_eq!(text.schema(), Some("text"));
        assert_eq!(text.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("hi!"));
    }

    #[semio_framework_async_macros::async_test]
    async fn manifest_lists_text_operators() {
        let json = build_manifest_json("text", "Text", "0.1.0", &neural_engine::ColdOwner::new(module_registry()), vec!["onStartup".into()], vec![], vec![FlowExtensionCommand { id: "text.showHelp".into(), title: "Text: Show Help".into() }], vec![]);
        assert!(json.contains("text.concat"));
        assert!(json.contains("\"operators\""));
    }

    #[semio_framework_async_macros::async_test]
    async fn evaluate_json_uppercases_text() {
        let text_value = pack::json::object([("$schema".to_string(), pack::json::Value::from("text")), ("value".to_string(), pack::json::Value::from("hi"))]);
        let input_json = pack::json::to_string(&pack::json::object([("text".to_string(), text_value)]));
        let out_json = evaluate_json(&neural_engine::ColdOwner::new(module_registry()), "text.upper", &input_json);
        let out = pack::json::parse(&out_json).unwrap();
        let text = out.get("text").expect("text channel");
        assert_eq!(text.get("$schema").and_then(pack::json::Value::as_str), Some("text"));
        assert_eq!(text.get("value").and_then(pack::json::Value::as_str), Some("HI"));
    }
}
// #endregion 🔖️Tests

// #region 🔖️ExtensionGuest
/// 🧩️ Runtime-installable flow extension bundle for `text`.
#[cfg(feature = "component-guest")]
mod extension_guest {
    use super::{extension_manifest_json, module_registry};
    use flow_extension_sdk::{evaluate_invoke_json, flow_extension_topic_contribution};
    use semio_framework::{Fault, FaultCode, FaultOrigin};
    use semio_framework_plugin::{ExecutionMode, ExtensionBundle};

    const FLOW_APP_ID: &str = "flow-play";
    const PROCEDURAL3D_APP_ID: &str = "procedural3d-play";
    const EXTENSION_ID: &str = "text";
    const EXTENSION_LABEL: &str = "Text";

    // 🚫️async: E1 pure — `extension_exports!` calls `bundle` outside an async context (macro requires
    // a plain sync fn). `.mode`/`.contributes_topic`/`.handler` are still `async fn` in
    // `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (out of this packet's
    // path_scope); bridged via `semio_framework::io::resolve_ready` — see this packet's lease-request.
    // See R9.
    fn bundle() -> ExtensionBundle {
        let manifest_json = extension_manifest_json();
        let flow_topic = flow_extension_topic_contribution(FLOW_APP_ID, EXTENSION_ID, EXTENSION_LABEL, "text", &manifest_json);
        let procedural3d_topic = flow_extension_topic_contribution(PROCEDURAL3D_APP_ID, EXTENSION_ID, EXTENSION_LABEL, "text", &manifest_json);
        let bundle = ExtensionBundle::new("flow-extension-text", EXTENSION_LABEL, "0.1.0").extends("flow");
        let bundle = semio_framework::io::resolve_ready(bundle.mode(ExecutionMode::Linked));
        let bundle = semio_framework::io::resolve_ready(bundle.contributes_topic(flow_topic.topic, flow_topic.payload));
        let bundle = semio_framework::io::resolve_ready(bundle.contributes_topic(procedural3d_topic.topic, procedural3d_topic.payload));
        semio_framework::io::resolve_ready(bundle.handler("evaluate", |req| {
            evaluate_invoke_json(&neural_engine::ColdOwner::new(module_registry()), req).map_err(|err| Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.evaluate.bad-request"), err))
        }))
    }

    #[test]
    fn bundle_identity_matches_catalogue_fixture() {
        let fixture = pack::json::parse(include_str!("../🧪️fixtures/🔣️package-identities.json")).unwrap();
        let bundle = bundle();
        assert_eq!(Some(bundle.manifest.extension_id.as_str()), fixture.get("text").and_then(|entry| entry.get("pluginId")).and_then(pack::json::Value::as_str));
        assert_eq!(bundle.manifest.topic_contributions.len(), 2);
        for contribution in &bundle.manifest.topic_contributions {
            assert_eq!(contribution.payload.get("extensionId").and_then(|value| value.as_str()), fixture.get("text").and_then(|entry| entry.get("flowId")).and_then(pack::json::Value::as_str));
        }
    }

    semio_framework_plugin::extension_exports!(bundle);
}
// #endregion 🔖️ExtensionGuest
