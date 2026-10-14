//! 🧩️ 🧩️ Generation2d play app commands command — `add-widget`.

use semio_s_artifact_procedural_2d_editor_config::Generation2dConfig;
use semio_s_artifact_procedural_2d_editor_config::Generation2dConfigMutation;
use semio_s_artifact_procedural_2d_mutations::host_add_widget;
use semio_s_artifact_procedural_2d_model::with_host;
use semio_s_artifact_procedural_2d_mutations::Generation2dMutation;

use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "add-widget")]
pub struct AddWidget {
    pub kind: String,
    pub neuron_kind: Option<String>,
    pub format: Option<String>,
    pub action: Option<String>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}


/// 🧩️ Descriptor JSON for `FlowHost::add_widget`: kind, neuronKind, and format or action when set.
fn add_widget_descriptor(payload: &AddWidget) -> String {
    let kind_pair = ("kind".to_string(), semio_framework_value::DslValue::String(payload.kind.clone()));
    if payload.kind == "neuron" {
        let neuron = ("neuronKind".to_string(), semio_framework_value::DslValue::String(payload.neuron_kind.clone().unwrap_or_else(|| "math.add".into())));
        return match (payload.format.clone(), payload.action.clone()) {
            (None, None) => semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::object([kind_pair, neuron])),
            (Some(format), None) => semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::object([kind_pair, neuron, ("format".to_string(), semio_framework_value::DslValue::String(format))])),
            (None, Some(action)) => semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::object([kind_pair, neuron, ("action".to_string(), semio_framework_value::DslValue::String(action))])),
            (Some(format), Some(action)) => semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::object([kind_pair, neuron, ("format".to_string(), semio_framework_value::DslValue::String(format)), ("action".to_string(), semio_framework_value::DslValue::String(action))])),
        };
    }
    if payload.kind == "inputSlider" {
        return semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::object([kind_pair, ("label".to_string(), semio_framework_value::DslValue::String(String::new()))]));
    }
    match (payload.format.clone(), payload.action.clone()) {
        (Some(format), None) => semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::object([kind_pair, ("format".to_string(), semio_framework_value::DslValue::String(format))])),
        (None, Some(action)) => semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::object([kind_pair, ("action".to_string(), semio_framework_value::DslValue::String(action))])),
        (Some(format), Some(action)) => semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::object([kind_pair, ("format".to_string(), semio_framework_value::DslValue::String(format)), ("action".to_string(), semio_framework_value::DslValue::String(action))])),
        (None, None) => semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::object([kind_pair])),
    }
}

/// 🕹️ No longer auto-selects the newly-added widget — no `Emit` channel writes `graph`'s selection
/// directly anymore (the framework owns it exclusively; ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub fn handle(payload: &AddWidget, doc: &ArtifactView<'_, Generation2dSnapshot>, _cfg: &ConfigView<'_, Generation2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.host_snapshot;
    let descriptor = add_widget_descriptor(payload);
    let operations = with_host(fixture, |host| host_add_widget(host, &descriptor, payload.x.unwrap_or(120.0), payload.y.unwrap_or(120.0))).unwrap_or_default();
    Ok(Emit { artifact_mutations: operations, ..Default::default() })
}

//#region 🧪️Tests
//#endregion 🧪️Tests
