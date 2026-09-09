use super::*;

#[semio_framework_async_macros::async_test]
async fn renders_node_graph_scene() {
    let graph = EquationGraph::default();
    let camera = EquationCamera::default();
    let node = render(&graph, &camera).expect("graph surface");
    let semio_framework_plugin::plugin_app_close_prelude::Component::Surface(props) = node.component else { panic!("graph must render a surface") };
    let scene: NodeGraphScene = semio_framework_ui_scene::decode(&props).expect("graph payload");
    let (nodes, edges) = workflow_json(&graph);
    assert_eq!(scene.editable, Some(true));
    assert_eq!(scene.nodes, nodes);
    assert_eq!(scene.edges, edges);
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_node_graph_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, MATH_PLAY_BODY_GRAPH);
    assert!(matches!(definition.surface_kind, SurfaceKind::NodeGraph));
}
