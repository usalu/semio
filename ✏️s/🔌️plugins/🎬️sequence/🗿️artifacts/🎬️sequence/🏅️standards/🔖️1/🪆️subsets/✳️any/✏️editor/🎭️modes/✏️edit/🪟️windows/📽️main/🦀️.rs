//! 📽️ Sequence play app — the main node-graph window: the editable step/flow canvas.

use super::config::SequenceMainWindowConfig;
use crate::editor::sequence::host_from_fixture;
use crate::SequenceFixture;
use semio_framework_plugin::{BuiltNode, LocalizedLabel, NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord, NodeGraphScene, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};
use semio_framework_os_kernel::Viewport2d;

//#region 🔖️Constants
pub const SEQUENCE_PLAY_WINDOW_MAIN: &str = "sequence-main";
pub const SEQUENCE_PLAY_BODY_MAIN: &str = "sequence.play.main";
const SEQUENCE_PLAY_SURFACE_MAIN: &str = "sequence.play.main";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: SEQUENCE_PLAY_WINDOW_MAIN.into(),
        label: LocalizedLabel::native("Sequence", "Sequenz"),
        body_key: SEQUENCE_PLAY_BODY_MAIN.into(),
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

//#region 🔖️Helpers
/// 🎯️ Single consumer (this window's `render`), so it lives here rather than the artifact engine.
fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint.split_once('@').map_or_else(|| (endpoint.to_string(), "next".into()), |(node, port)| (node.to_string(), port.to_string()))
}

fn fixture_to_workflow(fixture: &semio_framework_artifact_infinite_dag::DagFixture) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
    let nodes: Vec<NodeGraphNodeRecord> = fixture
        .nodes
        .iter()
        .map(|node| NodeGraphNodeRecord {
            id: node.id.clone(),
            label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
            x: node.x,
            y: node.y,
            width: node.width,
            height: node.height,
            inputs: node.inputs().iter().filter(|port| port.visible).map(|port| NodeGraphPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()), ..Default::default() }).collect(),
            outputs: node.outputs().iter().filter(|port| port.visible).map(|port| NodeGraphPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()), ..Default::default() }).collect(),
            ..Default::default()
        })
        .collect();
    let edges: Vec<NodeGraphEdgeRecord> = fixture
        .edges
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            NodeGraphEdgeRecord { id: edge.id.clone(), source_node_id, source_port_id, target_node_id, target_port_id, label: None }
        })
        .collect();
    (nodes, edges)
}
//#endregion 🔖️Helpers

//#region 🔖️Render
pub fn render(fixture: &SequenceFixture, config: &SequenceMainWindowConfig) -> UiAssemblyResult<BuiltNode> {
    let mut host = neural_engine::ColdOwner::new(host_from_fixture(fixture));
    host.layout_expanded_slots();
    let (nodes, edges) = fixture_to_workflow(&host.dag.fixture);
    let viewport = Viewport2d { x: config.camera.x, y: config.camera.y, zoom: config.camera.zoom };
    // 🕹️ `render` carries no `InteractionView` (ArtifactApp's breaking pass only added it to
    // `handle`/`copy_fragment`/`cut_operations` — see ticket 26/08/14's w3b-summary.md) and
    // `NodeGraphScene` has no `interaction_domain` field the wrapper could stamp post-render either
    // (unlike `UiNode::Tree` — see `stamp_and_cache_interaction_ui`), so `selection`/`hover` are left
    // at `NodeGraphScene::base`'s defaults (empty/none) — the canvas no longer paints a live
    // highlight until a future wave threads interaction into scene rendering. Flagged as a
    // discovered framework gap, not worked around here (same gap `space`'s workflow window carries).
    semio_framework_plugin::scene_surface(SEQUENCE_PLAY_SURFACE_MAIN, semio_framework_ui_contract::SurfaceKind::NodeGraph, &NodeGraphScene { editable: Some(true), ..NodeGraphScene::base(nodes, edges, viewport) })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
