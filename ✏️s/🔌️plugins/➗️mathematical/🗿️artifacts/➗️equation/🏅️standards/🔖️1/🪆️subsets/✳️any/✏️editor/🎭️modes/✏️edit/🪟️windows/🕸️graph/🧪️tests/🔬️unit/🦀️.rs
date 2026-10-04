use super::*;

#[semio_framework_async_macros::async_test]
async fn renders_node_graph_scene() {
    let graph = EquationGraph::default();
    let camera = EquationCamera::default();
    let node = render(&graph, &camera).expect("graph surface");
    let scene = semio_framework_plugin::artifact_app_laws::built_surface_scene::<NodeGraphScene>(&node).expect("the assembled graph scene: the spine with its node/edge lanes merged back in");
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
