use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_node_graph_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, SEQUENCE_VIEW_BODY_MAIN);
    assert!(matches!(definition.surface_kind, SurfaceKind::NodeGraph));
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_read_only_scene_for_the_default_document() {
    let document = neural_engine::ColdOwner::new(crate::default_snapshot());
    let scene = crate::sequence_working_scene(&document);
    let node = render(&scene).expect("viewer graph");
    let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("semantic graph") };
    let scene: NodeGraphScene = semio_framework_ui_scene::decode(props).expect("packed viewer scene");
    assert_eq!(scene.editable, Some(false));
    assert_eq!(scene.nodes.iter().map(|node| node.id.as_str()).collect::<Vec<_>>(), ["step-1", "step-2"]);
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire viewer graph");
}
