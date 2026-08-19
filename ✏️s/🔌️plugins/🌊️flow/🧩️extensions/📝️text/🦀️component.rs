//! 📝️ Flow text module: operators for text dictionaries.

use neural_engine::{channel_output, Atom, ChannelSpec, Dictionary, EvalError, Operator, OperatorImpl, OperatorInfo, Registry, Value};

// #region 🔖️Concat
/// 🔗️ Joins two text inputs.
pub struct Concat;

impl Operator for Concat {
    async fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("text", text_dictionary(format!("{}{}", read_channel_text(input, "a")?, read_channel_text(input, "b")?))))
    }
}
// #endregion 🔖️Concat

// #region 🔖️Upper
/// 🔠️ Uppercases a text input.
pub struct Upper;

impl Operator for Upper {
    async fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("text", text_dictionary(read_channel_text(input, "text")?.to_uppercase())))
    }
}
// #endregion 🔖️Upper

// #region 🔖️Helpers
async fn text_dictionary(value: String) -> Dictionary {
    Dictionary::with_schema("text").insert("value", Value::Atom(Atom::String(value)))
}

async fn read_channel_text(input: &Dictionary, key: &str) -> Result<String, EvalError> {
    input.get(key).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_str()).map(str::to_string).ok_or_else(|| EvalError::MissingInput(key.into()))
}

async fn text_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::text_default(id, "", &[operator_id])
}

async fn info(id: &str, name: &str, summary: &str, inputs: Vec<ChannelSpec>, output: ChannelSpec) -> OperatorInfo {
    OperatorInfo { id: id.into(), extension: "text".into(), name: name.into(), abbreviation: name.into(), icon: "emoji:📝️".into(), summary: summary.into(), inputs, outputs: vec![output], ..Default::default() }
}

// #endregion 🔖️Helpers

/// 📦️ Registers all text operators.
pub async fn register(registry: &mut Registry) {
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
pub async fn extension_manifest_json() -> String {
    use flow_extension_sdk::{build_manifest_json, FlowExtensionCommand};
    build_manifest_json("text", "Text", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![FlowExtensionCommand { id: "text.showHelp".into(), title: "Text: Show Help".into() }], vec![])
}

/// 🌊️ Builds an in-process operator registry for this extension.
pub async fn module_registry() -> Registry {
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
        let json = build_manifest_json("text", "Text", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![FlowExtensionCommand { id: "text.showHelp".into(), title: "Text: Show Help".into() }], vec![]);
        assert!(json.contains("text.concat"));
        assert!(json.contains("\"operators\""));
    }

    #[semio_framework_async_macros::async_test]
    async fn evaluate_json_uppercases_text() {
        let input = Dictionary::new().insert("text", Value::Dictionary(text_dictionary("hi".into())));
        let out_json = evaluate_json(&module_registry(), "text.upper", &serde_json::to_string(&input).unwrap());
        let out: Dictionary = serde_json::from_str(&out_json).unwrap();
        let text = out.get("text").and_then(|v| v.as_dictionary()).expect("text channel");
        assert_eq!(text.schema(), Some("text"));
        assert_eq!(text.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("HI"));
    }
}
// #endregion 🔖️Tests

// #region 🔖️ExtensionGuest
/// 🧩️ Runtime-installable flow extension bundle for `text`.
#[cfg(feature = "component-guest")]
mod extension_guest {
    use super::{extension_manifest_json, module_registry};
    use flow_extension_sdk::evaluate_json;
    use semio_framework::{Fault, FaultCode, FaultOrigin};
    use semio_framework_plugin::{ExecutionMode, ExtensionBundle};
    use serde::Deserialize;

    const FLOW_APP_ID: &str = "flow-play";
    const PROCEDURAL3D_APP_ID: &str = "procedural3d-play";
    const EXTENSION_ID: &str = "text";
    const EXTENSION_LABEL: &str = "Text";

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct EvaluateRequest {
        operator_id: String,
        input_json: String,
    }

    async fn flow_extension_contribution(app_id: &str, manifest_json: String) -> serde_json::Value {
        let icon_id = "text";
        let topic_payload = serde_json::json!({
            "appId": app_id,
            "extensionId": EXTENSION_ID,
            "label": EXTENSION_LABEL,
            "iconId": icon_id,
            "manifestJson": &manifest_json,
        });
        topic_payload
    }

    async fn bundle() -> ExtensionBundle {
        let manifest_json = extension_manifest_json();
        let flow_topic_payload = flow_extension_contribution(FLOW_APP_ID, manifest_json.clone());
        let procedural3d_topic_payload = flow_extension_contribution(PROCEDURAL3D_APP_ID, manifest_json);
        ExtensionBundle::new(EXTENSION_ID, EXTENSION_LABEL, "0.1.0")
            .extends("flow")
            .mode(ExecutionMode::Linked)
            .contributes_topic("flow.extension", flow_topic_payload)
            .contributes_topic("flow.extension", procedural3d_topic_payload)
            .handler("evaluate", |req| {
                let request: EvaluateRequest = serde_json::from_slice(req).map_err(|err| {
                    Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.evaluate.bad-request"), err.to_string())
                })?;
                Ok(evaluate_json(&module_registry(), &request.operator_id, &request.input_json).into_bytes())
            })
    }

    semio_framework_plugin::extension_exports!(bundle);
}
// #endregion 🔖️ExtensionGuest

