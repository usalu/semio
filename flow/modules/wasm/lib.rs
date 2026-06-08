//! 🔌 Shared wasm extension glue for flow modules.

use neural_engine::{inject_input_defaults, Dictionary, EvalError, NeuronKindInfo, Registry};
use serde::{Deserialize, Serialize};

// #region 🔖Manifest
/// 📋 `flow.module/v1` manifest document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowModuleManifestV1 {
    pub schema: String,
    pub id: String,
    pub name: String,
    pub version: String,
    pub activation_events: Vec<String>,
    pub contributes: FlowModuleContributesV1,
}

/// 🎁 Contributed extension surface.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowModuleContributesV1 {
    pub neuron_kinds: Vec<NeuronKindInfo>,
    #[serde(default)]
    pub widgets: Vec<FlowModuleWidgetV1>,
    #[serde(default)]
    pub commands: Vec<FlowModuleCommandV1>,
    #[serde(default)]
    pub settings: Vec<FlowModuleSettingV1>,
}

/// 🧩 Declared widget contribution.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowModuleWidgetV1 {
    pub kind: String,
    pub name: String,
    pub summary: String,
}

/// ⌘ Declared command contribution.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowModuleCommandV1 {
    pub id: String,
    pub title: String,
}

/// ⚙️ Declared setting contribution.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowModuleSettingV1 {
    pub id: String,
    #[serde(rename = "type")]
    pub setting_type: String,
    pub default: serde_json::Value,
    pub description: String,
}

/// 📦 Builds a `flow.module/v1` JSON manifest from registry catalogue metadata.
pub fn build_manifest_json(
    id: &str,
    name: &str,
    version: &str,
    registry: &Registry,
    activation_events: Vec<String>,
    widgets: Vec<FlowModuleWidgetV1>,
    commands: Vec<FlowModuleCommandV1>,
    settings: Vec<FlowModuleSettingV1>,
) -> String {
    let manifest = FlowModuleManifestV1 {
        schema: "flow.module/v1".into(),
        id: id.into(),
        name: name.into(),
        version: version.into(),
        activation_events,
        contributes: FlowModuleContributesV1 {
            neuron_kinds: registry.catalogue(),
            widgets,
            commands,
            settings,
        },
    };
    serde_json::to_string(&manifest).unwrap_or_else(|_| "{}".into())
}
// #endregion 🔖Manifest

// #region 🔖Evaluate
/// 🧮 Evaluates a neuron kind and returns JSON dictionary or `{ "error": ... }`.
pub fn evaluate_json(registry: &Registry, kind_id: &str, input_json: &str) -> String {
    let input: Dictionary = match serde_json::from_str(input_json) {
        Ok(d) => d,
        Err(err) => return serde_json::json!({ "error": err.to_string() }).to_string(),
    };
    match registry.get(kind_id) {
        Some(kind) => {
            let input = match registry.kind_info(kind_id) {
                Some(info) => inject_input_defaults(input, info),
                None => input,
            };
            match kind.evaluate(&input) {
            Ok(out) => serde_json::to_string(&out).unwrap_or_else(|_| "{}".into()),
            Err(err) => serde_json::json!({ "error": err.to_string() }).to_string(),
            }
        }
        None => serde_json::json!({ "error": EvalError::UnknownKind(kind_id.into()).to_string() }).to_string(),
    }
}
// #endregion 🔖Evaluate

// #region 🔖Command
/// ⌘ Stub command handler returning acknowledgement JSON.
pub fn command_json(command_id: &str, args_json: &str) -> String {
    serde_json::json!({ "ok": true, "commandId": command_id, "args": args_json }).to_string()
}
// #endregion 🔖Command

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use neural_engine::{Atom, Function, InputSpec, Value};

    struct Echo;

    impl Function for Echo {
        fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
            Ok(input.clone())
        }
    }

    #[test]
    fn manifest_lists_catalogue() {
        let mut reg = Registry::new();
        reg.register(
            NeuronKindInfo {
                id: "test.echo".into(),
                module: "test".into(),
                name: "Echo".into(),
                abbreviation: "Echo".into(),
                icon: "emoji:📣".into(),
                summary: "Echo".into(),
                inputs: vec![InputSpec::value("x")],
                outputs: vec!["x".into()],
                ..Default::default()
            },
            Box::new(Echo),
        );
        let json = build_manifest_json("test", "Test", "0.1.0", &reg, vec!["onStartup".into()], vec![], vec![], vec![]);
        assert!(json.contains("flow.module/v1"));
        assert!(json.contains("test.echo"));
    }

    #[test]
    fn evaluate_round_trips_dictionary() {
        let mut reg = Registry::new();
        reg.register(
            NeuronKindInfo {
                id: "test.echo".into(),
                module: "test".into(),
                name: "Echo".into(),
                abbreviation: "Echo".into(),
                icon: "emoji:📣".into(),
                summary: "Echo".into(),
                inputs: vec![InputSpec::value("x")],
                outputs: vec!["x".into()],
                ..Default::default()
            },
            Box::new(Echo),
        );
        let input = Dictionary::new().insert("number", Value::Atom(Atom::Decimal(2.0)));
        let out_json = evaluate_json(&reg, "test.echo", &serde_json::to_string(&input).unwrap());
        let out: Dictionary = serde_json::from_str(&out_json).unwrap();
        assert_eq!(out, input);
    }
}
// #endregion 🔖Tests
