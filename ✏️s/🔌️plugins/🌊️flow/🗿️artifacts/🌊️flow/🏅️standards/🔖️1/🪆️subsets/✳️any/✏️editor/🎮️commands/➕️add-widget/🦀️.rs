//! 🪟️ 🧩️ Flow play app commands command — `add-widget`.

use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::app::ChildEmit;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::{insert_node, SemioFlowMutation};
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use serde_json::json;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct AddWidget {
    pub kind: String,
    pub neuron_kind: Option<String>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

fn child_add_widget_fault(message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("flow.add-widget.child-delta-invalid"), message)
}

fn child_add_widget_mutation_from_descriptor(content: &SemioFlowSnapshot, descriptor: &semio_framework_artifact_flow_flow::WidgetDescriptor, kind_info: Option<&flow::neural::OperatorInfo>, x: f64, y: f64) -> Result<SemioFlowMutation, Fault> {
    content.nodes.len().checked_add(1).ok_or_else(|| child_add_widget_fault("Flow child node count overflow"))?;
    let id = semio_framework_artifact_flow_flow::descriptor_explicit_id(descriptor).unwrap_or_else(|| semio_framework_artifact_flow_flow::generated_widget_id(descriptor, content.nodes.iter().map(|node| node.id.as_str())).0);
    if content.nodes.iter().any(|node| node.id == id) {
        return Err(child_add_widget_fault(format!("widget id already exists: {id}")));
    }
    let widget = semio_framework_artifact_flow_flow::widget_from_descriptor_with_info(descriptor, id, kind_info);
    let layout = semio_framework_artifact_flow_flow::WidgetLayout { x, y };
    let node = crate::flow_content_node_from_working(&widget, Some(&layout));
    Ok(SemioFlowMutation::InsertNode(insert_node::InsertNode::new(node)))
}

fn child_add_widget_mutation(content: &SemioFlowSnapshot, descriptor_json: &str, x: f64, y: f64) -> Result<SemioFlowMutation, Fault> {
    let descriptor: semio_framework_artifact_flow_flow::WidgetDescriptor = flow::os_pack::json::from_json_str(descriptor_json).map_err(|error| child_add_widget_fault(error.to_string()))?;
    let registry = flow::flow_extension_registry();
    let kind_info = match &descriptor {
        semio_framework_artifact_flow_flow::WidgetDescriptor::Neuron { neuron_kind, .. } => registry.operator_info(neuron_kind),
        _ => None,
    };
    child_add_widget_mutation_from_descriptor(content, &descriptor, kind_info, x, y)
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the new widget used to also become the
/// selection here — selection is framework-owned `InteractionState` now, only ever mutated by the
/// framework's own injected `interactionSelect` handling, never by an app command's `Emit` (mirrors
/// note's `add-block`).
pub fn handle(payload: &AddWidget, doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    let descriptor = match payload.kind.as_str() {
        "neuron" => json!({ "kind": "neuron", "neuronKind": payload.neuron_kind.as_deref().unwrap_or("math.add") }).to_string(),
        "inputSlider" => json!({ "kind": "inputSlider", "label": "" }).to_string(),
        other => json!({ "kind": other }).to_string(),
    };
    let x = payload.x.unwrap_or(120.0);
    let y = payload.y.unwrap_or(120.0);
    let child_id = &doc.snapshot.content.child_id;
    let content = doc.children.typed_read::<SemioFlowSnapshot>("content", child_id)?;
    let mutation = child_add_widget_mutation(&content, &descriptor, x, y)?;
    Ok(Emit { child_emits: vec![ChildEmit::of::<SemioFlowSnapshot, _>("content", child_id, &[mutation])], ui_scope: UiDirtyScope::Full, ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
