use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_world_3d_surface_and_body_key() {
    let def = definition();
    assert_eq!(def.body_key, BODY_KEY);
    assert!(matches!(def.surface_kind, SurfaceKind::World3d));
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let snapshot = crate::standards::v1::subsets::any::schema::default_snapshot();
    let node = render(&snapshot).expect("viewer scene");
    let scene: World3dScene = semio_framework_plugin::testkit::built_surface_scene(&node).expect("assembled scene");
    assert!(scene.meshes_json.contains("/mesh/🧊️base.glb"));
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire viewer scene");
}
