//! 🕸️ Architect graph window — the program elements and their adjacencies as an undirected
//! node-graph surface, laid out on a circle.

use crate::artifacts::program::standards::v1::subsets::any::schema::inferences::undirected_edges;
use crate::artifacts::program::ProgramSnapshot;
use crate::editor::architect::chrome::empty_component_scene;
use crate::editor::architect::config::ArchitectConfig;
use semio_framework_plugin::{LocalizedLabel, NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord, NodeGraphScene, NodeGraphViewport, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};
use serde::{Deserialize, Serialize};

//#region 🔖️Constants
pub const ARCHITECT_WINDOW_GRAPH: &str = "architect-graph";
pub const ARCHITECT_BODY_GRAPH: &str = "architect.graph";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🏛️ Stitched into the app manifest by `crate::editor::architect::create_architect_app`.
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: ARCHITECT_WINDOW_GRAPH.into(),
        label: LocalizedLabel::native("Graph", "Graph"),
        body_key: ARCHITECT_BODY_GRAPH.into(),
        surface_kind: SurfaceKind::NodeGraph,
        icon_id: "architect-graph".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        // 🕹️ Populated post-hoc by `create_architect_app`'s `.window_kind_interactions(..)` call —
        // the "program" domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Camera
/// 🎥️ Ephemeral node-graph camera — parsed from `nodeGraphViewport`'s JSON payload and, on render,
/// reassembled from `ArchitectConfig`'s flattened `graph_camera_{x,y,zoom}` fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}
//#endregion 🔖️Camera

//#region 🔖️Render
pub async fn graph_media_json(program: &ProgramSnapshot, _camera: &GraphCamera) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
    let count = program.elements.len().max(1);
    let radius = 220.0;
    let center_x = 320.0;
    let center_y = 240.0;
    let nodes: Vec<NodeGraphNodeRecord> = program
        .elements
        .iter()
        .enumerate()
        .map(|(index, element)| {
            let angle = std::f64::consts::TAU * (index as f64) / (count as f64);
            NodeGraphNodeRecord {
                id: element.header.id.to_string(),
                label: Some(element.header.name.clone()),
                x: center_x + radius * angle.cos(),
                y: center_y + radius * angle.sin(),
                width: 108.0,
                height: 44.0,
                inputs: vec![NodeGraphPortRecord { id: "in".into(), label: None, ..Default::default() }],
                outputs: vec![NodeGraphPortRecord { id: "out".into(), label: None, ..Default::default() }],
                ..Default::default()
            }
        })
        .collect();
    let edges: Vec<NodeGraphEdgeRecord> = undirected_edges(program)
        .iter()
        .enumerate()
        .map(|(index, (source, target, weight))| NodeGraphEdgeRecord {
            id: format!("edge-{index}"),
            source_node_id: source.to_string(),
            source_port_id: "out".into(),
            target_node_id: target.to_string(),
            target_port_id: "in".into(),
            label: Some(format!("{weight:.1}")),
        })
        .collect();
    (nodes, edges)
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `render` carries no `InteractionView`
/// and `NodeGraphScene` has no `interaction_domain` field the wrapper could stamp post-render either
/// (unlike `UiNode::Tree`) — `selection`/`hover` are left at `NodeGraphScene::base`'s defaults
/// (empty/none), matching `dag`'s main window's and `space`'s workflow window's identical gap.
pub async fn render(program: &ProgramSnapshot, cfg: &ArchitectConfig) -> UiNode {
    let camera = GraphCamera { x: cfg.graph_camera_x, y: cfg.graph_camera_y, zoom: cfg.graph_camera_zoom };
    let (nodes, edges) = graph_media_json(program, &camera);
    let viewport = NodeGraphViewport { x: camera.x, y: camera.y, zoom: camera.zoom };
    let mut scene = empty_component_scene(ARCHITECT_BODY_GRAPH, SurfaceKind::NodeGraph);
    scene.node_graph = Some(NodeGraphScene { editable: Some(true), capabilities_json: Some(r#"{"directedness":"undirected"}"#.into()), ..NodeGraphScene::base(nodes, edges, viewport) });
    UiNode::ComponentScene(scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::sample_plugin;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_node_graph_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, ARCHITECT_BODY_GRAPH);
        assert!(matches!(definition.surface_kind, SurfaceKind::NodeGraph));
    }

    #[semio_framework_async_macros::async_test]
    async fn the_graph_body_emits_a_node_graph_scene() {
        let json = serde_json::to_string(&render(&sample_plugin(), &ArchitectConfig::default())).expect("json");
        assert!(json.contains("node-graph"));
    }

    #[semio_framework_async_macros::async_test]
    async fn every_element_becomes_a_node_and_every_adjacency_an_edge() {
        let program = sample_plugin();
        let (nodes, edges) = graph_media_json(&program, &GraphCamera { x: 0.0, y: 0.0, zoom: 1.0 });
        assert_eq!(nodes.len(), program.elements.len());
        assert_eq!(edges.len(), undirected_edges(&program).len());
    }
}
//#endregion 🧪️Tests
