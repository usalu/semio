
use super::*;
use crate::editor::layout::testkit::{layout_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn document_lists_sample_pages() {
    let mut app = layout_app().await;
    let json = render_body(&mut app, LAYOUT_PLAY_BODY_DOCUMENT).await;
    assert!(json.contains("layout-document.page.page-1"));
    assert!(json.contains("Page 1"));
}

#[semio_framework_async_macros::async_test]
async fn document_tree_has_nine_sections() {
    let mut app = layout_app().await;
    let json = render_body(&mut app, LAYOUT_PLAY_BODY_DOCUMENT).await;
    for section_id in
        ["layout-document.document", "layout-document.spreads", "layout-document.pages", "layout-document.frames", "layout-document.parentPages", "layout-document.layers", "layout-document.stories", "layout-document.links", "layout-document.styles"]
    {
        assert!(json.contains(section_id), "missing section {section_id}");
    }
}

#[semio_framework_async_macros::async_test]
async fn layout_labels_resolve_native_english_by_default() {
    let mut app = layout_app().await;
    let json = render_body(&mut app, LAYOUT_PLAY_BODY_DOCUMENT).await;
    assert!(json.contains("\"Frames\""));
    assert!(json.contains("\"Layers\""));
    assert!(!json.contains("Rahmen"));
}

#[semio_framework_async_macros::async_test]
async fn layout_labels_translate_document_tree_in_german() {
    use crate::editor::layout::LayoutCommand;
    use crate::editor::layout::testkit::dispatch;
    let mut app = layout_app().await;
    let json = render_body(&mut app, LAYOUT_PLAY_BODY_DOCUMENT).await;
    assert!(json.contains("\"Rahmen\""));
    assert!(json.contains("\"Ebenen\""));
    assert!(!json.contains("\"Frames\""));
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_document_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    assert_eq!(definition.body_key.as_deref(), Some(LAYOUT_PLAY_BODY_DOCUMENT));
}
