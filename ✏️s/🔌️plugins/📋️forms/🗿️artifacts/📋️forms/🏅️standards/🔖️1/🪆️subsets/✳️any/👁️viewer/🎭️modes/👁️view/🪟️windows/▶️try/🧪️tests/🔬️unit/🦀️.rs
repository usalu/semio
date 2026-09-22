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
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).unwrap();
    assert!(json.contains("\"container\""));
}

#[semio_framework_async_macros::async_test]
async fn render_falls_back_to_a_placeholder_for_an_empty_document() {
    // 🪹️ A document with NO steps — `empty_forms_snapshot()` is the minimal ONE-step seed, which
    // renders its step column and never reaches the placeholder branch.
    let document = crate::forms_snapshot_with_state(crate::FORMS_DOCUMENT_SCHEMA.into(), "forms".into(), "1".into(), None, &[]);
    let node = render(&document).unwrap();
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).unwrap();
    assert!(json.contains("No steps"));
}
