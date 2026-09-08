
use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_canvas2d_try_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
    assert!(matches!(def.surface_kind, SurfaceKind::Canvas2d));
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_node_for_the_default_document() {
    let document = crate::schema::building_component_spec();
    let node = render(&document).unwrap();
    let json = serde_json::to_string(&node).unwrap();
    assert!(json.contains("\"container\""));
}

#[semio_framework_async_macros::async_test]
async fn render_falls_back_to_a_placeholder_for_an_empty_document() {
    let document = crate::schema::empty_forms_snapshot();
    let node = render(&document).unwrap();
    let json = serde_json::to_string(&node).unwrap();
    assert!(json.contains("No steps"));
}
