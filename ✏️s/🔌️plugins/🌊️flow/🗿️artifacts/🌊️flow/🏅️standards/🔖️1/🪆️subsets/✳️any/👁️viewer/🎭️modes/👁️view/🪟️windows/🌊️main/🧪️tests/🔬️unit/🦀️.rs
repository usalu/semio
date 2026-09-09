use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_node_graph_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, BODY_KEY);
    assert!(matches!(definition.surface_kind, SurfaceKind::NodeGraph));
}

#[semio_framework_async_macros::async_test]
async fn renders_node_graph_scene_for_the_default_document() {
    let document = FlowSnapshot::default();
    let node = render(&document).expect("render");
    let json = serde_json::to_string(&node).expect("render json");
    assert!(json.contains("node-graph"));
}
