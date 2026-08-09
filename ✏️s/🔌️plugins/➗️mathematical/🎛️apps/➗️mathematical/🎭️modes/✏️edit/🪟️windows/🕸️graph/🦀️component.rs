//! 🕸️ Mathematical play app — the graph window: the editable node-graph canvas.

use crate::artifacts::mathematical::engine::{empty_component_scene, workflow_json};
use crate::artifacts::mathematical::{MathematicalCamera, MathematicalGraph};
use semio_framework_plugin::{LocalizedLabel, NodeGraphScene, NodeGraphViewport, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const MATH_PLAY_WINDOW_GRAPH: &str = "math-graph";
pub const MATH_PLAY_BODY_GRAPH: &str = "mathematical.play.graph";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::mathematical::create_mathematical_app`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: MATH_PLAY_WINDOW_GRAPH.into(),
        label: LocalizedLabel::native("Graph", "Graph"),
        body_key: MATH_PLAY_BODY_GRAPH.into(),
        surface_kind: SurfaceKind::NodeGraph,
        icon_id: "math-graph".into(),
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

//#region 🔖️Render
pub fn render(graph: &MathematicalGraph, camera: &MathematicalCamera) -> UiNode {
    let (nodes, edges) = workflow_json(graph);
    let viewport = NodeGraphViewport { x: camera.x, y: camera.y, zoom: camera.zoom };
    let mut scene = empty_component_scene(MATH_PLAY_BODY_GRAPH, SurfaceKind::NodeGraph);
    scene.node_graph = Some(NodeGraphScene { editable: Some(true), ..NodeGraphScene::base(nodes, edges, viewport) });
    UiNode::ComponentScene(scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_node_graph_scene() {
        let json = serde_json::to_string(&render(&MathematicalGraph::default(), &MathematicalCamera::default())).unwrap();
        assert!(json.contains("node-graph"));
    }

    #[test]
    fn definition_declares_the_node_graph_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, MATH_PLAY_BODY_GRAPH);
        assert!(matches!(definition.surface_kind, SurfaceKind::NodeGraph));
    }
}
//#endregion 🧪️Tests
