//! 🪟️ 🧩️ Flow play app commands command — `add-widget`.

use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use flow::FlowEvalSession;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::app::ChildEmit;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::flow::schema::mutations::{insert_node, SemioFlowMutation};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
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
    let (widgets, synapses, layout) = crate::artifacts::flow::working_from_flow_content_snapshot(content);
    let fixture = flow::FlowFixture { schema: flow::FLOW_DOCUMENT_SCHEMA.into(), camera: Default::default(), widgets, synapses, layout };
    let mut host = flow::flow_host_with_session(&fixture, session);
    crate::editor::flow::seed_host_catalogue(&mut host, &config.catalogue_sections_json);
    crate::editor::flow::apply_canvas_options(&mut host, config);
    let id = host.add_widget(descriptor, x, y).map_err(|error| child_add_widget_fault(error.to_string()))?;
    let post = crate::artifacts::flow::flow_content_snapshot_from_working(&host.fixture.widgets, &host.fixture.synapses, &host.fixture.layout);
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
        child_emits: vec![ChildEmit::of::<SemioFlowSnapshot, _>("content", child_id, vec![mutation])],
        ui_scope: UiDirtyScope::Full,
        ..Default::default()
    })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::flow::testkit::{dispatch, flow_app};
    use crate::editor::flow::FlowCommand;
    use store::{ArtifactPack, SpaceMember};

    #[semio_framework_async_macros::async_test]
    async fn add_widget_dispatches_one_typed_child_edit_without_repointing_parent_content() {
        let mut app = flow_app().await;
        let parent_before = app.snapshot().expect("snapshot");
        let child_id = parent_before.content.child_id.clone();
        let content_before = SemioFlowSnapshot::decode_pack(&app.child_store("content", &child_id).await.expect("Flow child").document_pack_bytes().await.expect("Flow child pack")).expect("Flow child snapshot");
        let result = dispatch(&mut app, FlowCommand::AddWidget(AddWidget { kind: "inputNote".into(), neuron_kind: None, x: Some(40.0), y: Some(40.0) })).await;
        let repeated = dispatch(&mut app, FlowCommand::AddWidget(AddWidget { kind: "inputNote".into(), neuron_kind: None, x: Some(50.0), y: Some(51.0) })).await;
        let parent_after = app.snapshot().expect("snapshot");
        let content_after = SemioFlowSnapshot::decode_pack(&app.child_store("content", &child_id).await.expect("Flow child").document_pack_bytes().await.expect("Flow child pack")).expect("Flow child snapshot");
        assert_eq!(result.mutations.len(), 1, "addWidget must expose one child kernel mutation");
        assert_eq!(repeated.mutations.len(), 1, "a reconstructed host must expose one child kernel mutation for a repeated kind");
        assert_eq!(result.inverse_group.member_edits.len(), 1, "addWidget must dispatch one typed child edit");
        assert_eq!(repeated.inverse_group.member_edits.len(), 1, "a repeated kind must dispatch one typed child edit");
        assert_eq!(parent_after.content, parent_before.content, "addWidget must preserve the exact parent content coordinate");
        assert_eq!(content_after.nodes.len(), content_before.nodes.len() + 2);
        let inserted = &content_after.nodes[content_before.nodes.len()..];
        assert_eq!((inserted[0].position.x, inserted[0].position.y), (40.0, 40.0));
        assert_eq!((inserted[1].position.x, inserted[1].position.y), (50.0, 51.0));
        assert_eq!((inserted[0].id.as_str(), inserted[1].id.as_str()), ("note_2", "note_3"));
    }

    #[semio_framework_async_macros::async_test]
    async fn rename_rejects_blank_unchanged_and_taken_ids() {
        let mut app = flow_app().await;
        for value in ["", " ", "slider"] {
            let result = dispatch(&mut app, FlowCommand::RenameFlowWidget(crate::editor::flow::commands::rename_flow_widget::RenameFlowWidget { old_id: "slider".into(), value: value.into() })).await;
            assert!(result.mutations.is_empty(), "rename to {value:?} must be a no-operation");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn patch_flow_widgets_parses_the_raw_value_string_into_the_slider() {
        let mut app = flow_app().await;
        dispatch(&mut app, FlowCommand::PatchFlowWidgets(crate::editor::flow::commands::patch_flow_widgets::PatchFlowWidgets { widget_ids: vec!["slider".into()], field: "value".into(), value: "7.5".into() })).await;
        let patched = app.snapshot().expect("snapshot");
        let patched_widgets = patched.to_fixture().widgets;
        assert!(patched_widgets.iter().any(|widget| matches!(widget, flow::Widget::InputSlider { id, value, .. } if id == "slider" && (value - 7.5).abs() < f64::EPSILON)), "slider must carry the parsed value: {patched_widgets:?}");
    }
}
//#endregion 🧪️Tests
