use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_canvas_2d_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, WIRES_VIEW_BODY_CANVAS);
    assert!(matches!(definition.surface_kind, SurfaceKind::Canvas2d));
}

#[semio_framework_async_macros::async_test]
async fn renders_canvas_scene_for_the_empty_document() {
    let document = crate::empty_wires_snapshot();
    let node = render(&document).expect("viewer canvas");
    let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("canvas surface") };
    let scene: Canvas2dScene = semio_framework_ui_scene::decode(props).expect("packed canvas");
    let json = scene.layers_json;
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire viewer tree");
    assert!(serde_json::from_str::<serde_json::Value>(&json).expect("layer oracle").is_array());
}

#[semio_framework_async_macros::async_test]
async fn renders_canvas_scene_for_the_metabolism_example() {
    let document = crate::schema::metabolism_wires_example_snapshot().expect("valid metabolism fixture mutations");
    let node = render(&document).expect("viewer canvas");
    let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("canvas surface") };
    let scene: Canvas2dScene = semio_framework_ui_scene::decode(props).expect("packed canvas");
    let json = scene.layers_json;
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("retire viewer tree");
    assert!(serde_json::from_str::<serde_json::Value>(&json).expect("layer oracle").is_array());
    assert!(json.contains("Demo") || json.contains("Metabolism") || json.contains("Topic"));
}
