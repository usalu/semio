//! 📽️ Sequence play app — the main node-graph window: the editable step/flow canvas.

use crate::editor::sequence::config::SequenceConfig;
use crate::editor::sequence::SEQUENCE_PLAY_APP_ID;
use crate::editor::sequence::host_from_snapshot;
use crate::artifacts::sequence::SequenceSnapshot;
use semio_framework_plugin::{build_node_graph_scene, LocalizedLabel, NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord, NodeGraphScene, NodeGraphViewport, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const SEQUENCE_PLAY_WINDOW_MAIN: &str = "sequence-main";
pub const SEQUENCE_PLAY_BODY_MAIN: &str = "sequence.play.main";
const SEQUENCE_PLAY_SURFACE_MAIN: &str = "sequence.play.main";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> WindowKindDefinition {
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
async fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint.split_once('@').map_or_else(|| (endpoint.to_string(), "next".into()), |(node, port)| (node.to_string(), port.to_string()))
}

async fn fixture_to_workflow(fixture: &infinite_board_port_directed_dag::DagFixture) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
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
pub async fn render(fixture: &SequenceSnapshot, config: &SequenceConfig) -> UiNode {
    let mut host = host_from_snapshot(fixture);
    host.layout_expanded_slots();
    let (nodes, edges) = fixture_to_workflow(&host.dag.fixture);
    let viewport = NodeGraphViewport { x: config.camera.x, y: config.camera.y, zoom: config.camera.zoom };
    // 🕹️ `render` carries no `InteractionView` (ArtifactApp's breaking pass only added it to
    // `handle`/`copy_fragment`/`cut_operations` — see ticket 26/08/14's w3b-summary.md) and
    // `NodeGraphScene` has no `interaction_domain` field the wrapper could stamp post-render either
    // (unlike `UiNode::Tree` — see `stamp_and_cache_interaction_ui`), so `selection`/`hover` are left
    // at `NodeGraphScene::base`'s defaults (empty/none) — the canvas no longer paints a live
    // highlight until a future wave threads interaction into scene rendering. Flagged as a
    // discovered framework gap, not worked around here (same gap `space`'s workflow window carries).
    build_node_graph_scene(SEQUENCE_PLAY_SURFACE_MAIN, SEQUENCE_PLAY_APP_ID, NodeGraphScene { editable: Some(true), ..NodeGraphScene::base(nodes, edges, viewport) })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::sequence::testkit::{new_app, render as render_body};

    #[test]
    async fn renders_node_graph_scene() {
        let mut app = new_app();
        assert!(render_body(&mut app, SEQUENCE_PLAY_BODY_MAIN).contains("node-graph"));
    }

    #[test]
    async fn definition_declares_the_node_graph_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, SEQUENCE_PLAY_BODY_MAIN);
        assert!(matches!(definition.surface_kind, SurfaceKind::NodeGraph));
    }

    #[test]
    async fn split_endpoint_defaults_port_to_next() {
        assert_eq!(split_endpoint("node@port"), ("node".to_string(), "port".to_string()));
        assert_eq!(split_endpoint("node"), ("node".to_string(), "next".to_string()));
    }
}
//#endregion 🧪️Tests
