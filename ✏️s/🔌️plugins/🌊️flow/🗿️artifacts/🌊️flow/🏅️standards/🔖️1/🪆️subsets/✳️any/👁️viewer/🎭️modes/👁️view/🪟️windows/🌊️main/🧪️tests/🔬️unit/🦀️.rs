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
    let tree = semio_framework_plugin::app::built_to_component_tree(node);
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("render projection");
    assert!(json.contains("node-graph"));
}
