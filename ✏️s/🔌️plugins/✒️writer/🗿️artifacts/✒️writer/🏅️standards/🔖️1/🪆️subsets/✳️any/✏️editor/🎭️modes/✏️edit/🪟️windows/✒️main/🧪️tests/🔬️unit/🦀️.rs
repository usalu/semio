use super::*;
use crate::editor::writer::unit_tests::context::{main_window_measures, new_app, render as render_body};
use semio_framework_plugin::PluginApp;

#[semio_framework_async_macros::async_test]
async fn renders_text_editor_scene() {
    let mut app = new_app().await;
    assert!(render_body(&mut app, WRITER_PLAY_BODY_MAIN).await.contains("text-editor"));
}

/// 🚚️ The text-editor scene is a packed doc plus out-of-doc payload lanes, never camelCase JSON keys in
/// the projection, so the law reads the assembled scene the way a render host does.
#[semio_framework_async_macros::async_test]
async fn scene_emits_placeholders_selectable_spans_and_newline_gates_for_jack() {
    let mut app = new_app().await;
    let node = app.render(WRITER_PLAY_BODY_MAIN, Some(&crate::document_dsl::jack_example_json()), &crate::editor::writer::unit_tests::context::main_window_view()).await.expect("render");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(node).expect("render JSON");
    let scene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes::<TextEditorScene>(&json).expect("text-editor scene");
    assert!(scene.placeholders_json.as_deref().is_some_and(|value| !value.is_empty()), "placeholders: {:?}", scene.placeholders_json);
    assert!(scene.selectable_spans_json.as_deref().is_some_and(|value| !value.is_empty()), "selectable spans: {:?}", scene.selectable_spans_json);
    assert!(scene.newline_gates_json.as_deref().is_some_and(|value| !value.is_empty()), "newline gates: {:?}", scene.newline_gates_json);
}

#[semio_framework_async_macros::async_test]
async fn window_measures_expose_font_line_height_tab_and_toggle() {
    let mut app = new_app().await;
    let measures = main_window_measures(&mut app).await;
    assert_eq!(measures.len(), 4);
    assert!(measures.iter().any(|m| matches!(m, WindowMeasure::Toggle { id, .. } if id == "writer-line-numbers-measure")));
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_text_editor_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, WRITER_PLAY_BODY_MAIN);
    assert!(matches!(definition.surface_kind, SurfaceKind::TextEditor));
    assert!(definition.options.measures.is_empty(), "measures are config-derived per frame, never frozen into the manifest");
}
