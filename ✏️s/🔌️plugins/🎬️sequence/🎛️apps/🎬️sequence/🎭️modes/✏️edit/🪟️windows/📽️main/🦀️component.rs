//! 📽️ Sequence play app — the main node-graph window: the editable step/flow canvas.

use crate::apps::sequence::config::SequenceConfig;
use crate::apps::sequence::SEQUENCE_PLAY_APP_ID;
use crate::artifacts::sequence::engine::host_from_snapshot;
use crate::artifacts::sequence::SequenceSnapshot;
use semio_framework_plugin::{build_node_graph_scene, LocalizedLabel, NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord, NodeGraphScene, NodeGraphViewport, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};

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
        document_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Helpers
/// 🎯️ Single consumer (this window's `render`), so it lives here rather than the artifact engine.
fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint.split_once('@').map_or_else(|| (endpoint.to_string(), "next".into()), |(node, port)| (node.to_string(), port.to_string()))
}

fn fixture_to_workflow(fixture: &infinite_board_port_directed_dag::DagFixture) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
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
pub fn render(fixture: &SequenceSnapshot, config: &SequenceConfig) -> UiNode {
    let mut host = host_from_snapshot(fixture);
    host.layout_expanded_slots();
    let (nodes, edges) = fixture_to_workflow(&host.dag.fixture);
    let viewport = NodeGraphViewport { x: config.camera.x, y: config.camera.y, zoom: config.camera.zoom };
    let selection = config.selected_step_ids.clone();
    build_node_graph_scene(SEQUENCE_PLAY_SURFACE_MAIN, SEQUENCE_PLAY_APP_ID, NodeGraphScene { editable: Some(true), selection, ..NodeGraphScene::base(nodes, edges, viewport) })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::sequence::testkit::{new_app, render as render_body};

    #[test]
    fn renders_node_graph_scene() {
        let mut app = new_app();
        assert!(render_body(&mut app, SEQUENCE_PLAY_BODY_MAIN).contains("node-graph"));
    }

    #[test]
    fn definition_declares_the_node_graph_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, SEQUENCE_PLAY_BODY_MAIN);
        assert!(matches!(definition.surface_kind, SurfaceKind::NodeGraph));
    }

    #[test]
    fn split_endpoint_defaults_port_to_next() {
        assert_eq!(split_endpoint("node@port"), ("node".to_string(), "port".to_string()));
        assert_eq!(split_endpoint("node"), ("node".to_string(), "next".to_string()));
    }
}
//#endregion 🧪️Tests
