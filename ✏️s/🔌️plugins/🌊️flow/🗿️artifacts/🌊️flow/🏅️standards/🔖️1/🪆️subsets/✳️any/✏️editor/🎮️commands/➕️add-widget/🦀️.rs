//! 🪟️ 🧩️ Flow play app commands command — `add-widget`.

use crate::{op::FlowMutation, FlowSnapshot};
use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use flow::FlowEvalSession;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::app::ChildEmit;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::{insert_node, SemioFlowMutation};
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use serde_json::json;
use semio_framework_value_derive::{FromValue, ToValue};

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

fn child_add_widget_mutation(content: &SemioFlowSnapshot, config: &FlowConfig, session: &FlowEvalSession, descriptor: &str, x: f64, y: f64) -> Result<SemioFlowMutation, Fault> {
    let (widgets, synapses, layout) = crate::working_from_flow_content_snapshot(content);
    let fixture = semio_framework_artifact_flow_flow::FlowFixture { schema: semio_framework_artifact_flow_flow::FLOW_DOCUMENT_SCHEMA.into(), camera: Default::default(), widgets, synapses, layout };
    let mut host = flow::flow_host_with_session(&fixture, session);
    crate::editor::flow::seed_host_catalogue(&mut host, &config.catalogue_sections_json);
    crate::editor::flow::apply_canvas_options(&mut host, config);
    let id = host.add_widget(descriptor, x, y).map_err(|error| child_add_widget_fault(error.to_string()))?;
    let post = crate::flow_content_snapshot_from_working(&host.fixture.widgets, &host.fixture.synapses, &host.fixture.layout);
    let expected_len = content.nodes.len().checked_add(1).ok_or_else(|| child_add_widget_fault("Flow child node count overflow"))?;
    if post.schema != content.schema || post.nodes.len() != expected_len || post.nodes[..content.nodes.len()] != content.nodes || post.edges != content.edges {
        return Err(child_add_widget_fault("Flow host add-widget produced a delta outside one appended typed node"));
    }
    let node = post.nodes.last().cloned().ok_or_else(|| child_add_widget_fault("Flow host add-widget produced no typed node"))?;
    if node.id != id || node.position.x != x || node.position.y != y {
        return Err(child_add_widget_fault("Flow host add-widget did not preserve the exact inserted identity and position"));
    }
    Ok(SemioFlowMutation::InsertNode(insert_node::InsertNode::new(node)))
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the new widget used to also become the
/// selection here — selection is framework-owned `InteractionState` now, only ever mutated by the
/// framework's own injected `interactionSelect` handling, never by an app command's `Emit` (mirrors
/// note's `add-block`).
pub fn handle(payload: &AddWidget, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    let descriptor = match payload.kind.as_str() {
        "neuron" => json!({ "kind": "neuron", "neuronKind": payload.neuron_kind.as_deref().unwrap_or("math.add") }).to_string(),
        "inputSlider" => json!({ "kind": "inputSlider", "label": "" }).to_string(),
        other => json!({ "kind": other }).to_string(),
    };
    let x = payload.x.unwrap_or(120.0);
    let y = payload.y.unwrap_or(120.0);
    let child_id = &doc.snapshot.content.child_id;
    let content = doc.children.typed_read::<SemioFlowSnapshot>("content", child_id)?;
    let mutation = child_add_widget_mutation(&content, cfg.snapshot, session, &descriptor, x, y)?;
    Ok(Emit {
        child_emits: vec![ChildEmit::of::<SemioFlowSnapshot, _>("content", child_id, &[mutation])],
        ui_scope: UiDirtyScope::Full,
        ..Default::default()
    })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
