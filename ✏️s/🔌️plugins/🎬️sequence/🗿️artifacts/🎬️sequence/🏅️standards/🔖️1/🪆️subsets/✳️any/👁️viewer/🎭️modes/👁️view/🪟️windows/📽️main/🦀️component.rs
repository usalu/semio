//! 📽️ Sequence viewer — the Main window: a read-only node-graph render of the live steps/edges, built
//! directly from the artifact-level `SequenceSnapshot::to_fixture()`/`SequenceStep`/`SequenceEdge`
//! pure types — this file itself imports nothing from the sibling editor module
//! (`policyViewerPurityBreaches` forbids it outright). No selection, no engagement, no drag/connect
//! utilities: a viewer has no utilities that edit and emits no mutations by construction (`ViewEmit`).
//! Node positions come straight off each step's persisted `x`/`y` — unlike the editor's own Main
//! window, this never runs the layered-layout/ghost/selection machinery `SequenceHost` (an editor-only
//! type) provides, since a viewer never needs to lay anything out interactively.

use crate::artifacts::sequence::{SequenceSnapshot, SequenceStep};
use semio_framework_plugin::{build_node_graph_scene, LocalizedLabel, NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphScene, NodeGraphViewport, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const SEQUENCE_VIEW_WINDOW_MAIN: &str = "sequence-view-main";
pub const SEQUENCE_VIEW_BODY_MAIN: &str = "sequence.view.main";
const SEQUENCE_VIEW_SURFACE_MAIN: &str = "sequence.view.main";
/// 👁️ Read-only counterpart of the editor's `SEQUENCE_PLAY_APP_ID` controller id — kept distinct so
/// a viewer session's node-graph controller can never be mistaken for an editor session's.
const SEQUENCE_VIEW_CONTROLLER_ID: &str = "sequence-view";
/// 👁️ A viewer has no persisted per-session camera (`Config = NoConfig`) — hardcoded default,
/// documented as an intentional simplification, not a bug (mirrors `📐️cad`'s viewer camera/sun
/// defaults).
const SEQUENCE_VIEW_DEFAULT_WIDTH: f64 = 160.0;
const SEQUENCE_VIEW_DEFAULT_HEIGHT: f64 = 56.0;
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: SEQUENCE_VIEW_WINDOW_MAIN.into(),
        label: LocalizedLabel::native("Sequence", "Sequenz"),
        body_key: SEQUENCE_VIEW_BODY_MAIN.into(),
        surface_kind: SurfaceKind::NodeGraph,
        icon_id: "list-ordered".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
async fn step_node(step: &SequenceStep) -> NodeGraphNodeRecord {
    NodeGraphNodeRecord {
        id: step.id.clone(),
        label: Some(format!("{} ({})", step.id, step.kind)),
        x: step.x,
        y: step.y,
        width: SEQUENCE_VIEW_DEFAULT_WIDTH,
        height: SEQUENCE_VIEW_DEFAULT_HEIGHT,
        inputs: Vec::new(),
        outputs: Vec::new(),
        instance_id: None,
        plugin_id: None,
        app_id: None,
        icon: None,
    }
}

/// 👁️ Pure `SequenceSnapshot -> UiNode` read: default viewport (a viewer has no persisted
/// per-session camera), no selection/drag overlay, `editable: Some(false)` (contract §2.2's
/// structural read-only guarantee, mirrored here at the scene level too).
pub async fn render(document: &SequenceSnapshot) -> UiNode {
    let fixture = document.to_fixture();
    let nodes: Vec<NodeGraphNodeRecord> = fixture.steps.iter().map(step_node).collect();
    let edges: Vec<NodeGraphEdgeRecord> = fixture.edges.iter().map(|edge| NodeGraphEdgeRecord { id: edge.id.clone(), source_node_id: edge.from.clone(), source_port_id: String::new(), target_node_id: edge.to.clone(), target_port_id: String::new(), label: None }).collect();
    let viewport = NodeGraphViewport { x: 0.0, y: 0.0, zoom: 1.0 };
    build_node_graph_scene(SEQUENCE_VIEW_SURFACE_MAIN, SEQUENCE_VIEW_CONTROLLER_ID, NodeGraphScene { editable: Some(false), ..NodeGraphScene::base(nodes, edges, viewport) })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_node_graph_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, SEQUENCE_VIEW_BODY_MAIN);
        assert!(matches!(definition.surface_kind, SurfaceKind::NodeGraph));
    }

    #[semio_framework_async_macros::async_test]
    async fn render_produces_a_read_only_scene_for_the_default_document() {
        let document = crate::artifacts::sequence::default_snapshot();
        let node = render(&document);
        let json = serde_json::to_string(&node).unwrap_or_default();
        assert!(json.contains("\"editable\":false"));
        assert!(json.contains("step-1"));
        assert!(json.contains("step-2"));
    }
}
//#endregion 🧪️Tests
