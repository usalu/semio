
use super::*;

#[semio_framework_async_macros::async_test]
async fn renders_canvas_2d_scene() {
    let geometry = EquationGeometry::default();
    let node = render(&geometry).expect("geometry surface");
    let semio_framework_plugin::plugin_app_close_prelude::Component::Surface(props) = node.component else { panic!("geometry must render a surface") };
    let scene: Canvas2dScene = semio_framework_ui_scene::decode(&props).expect("geometry payload");
    assert_eq!(scene.layers_json, geometry_layers_json(&geometry));
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_canvas_2d_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, MATH_PLAY_BODY_GEOMETRY);
    assert!(matches!(definition.surface_kind, SurfaceKind::Canvas2d));
}
