use super::*;
use crate::editor::layout::testkit::{layout_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn catalogue_lists_frame_kinds() {
    let mut app = layout_app().await;
    let json = render_body(&mut app, LAYOUT_PLAY_BODY_CATALOGUE).await;
    assert!(json.contains("layout-catalogue.rect"));
    assert!(json.contains("Text Frame"));
}

#[semio_framework_async_macros::async_test]
async fn catalogue_items_are_draggable() {
    let mut app = layout_app().await;
    let json = render_body(&mut app, LAYOUT_PLAY_BODY_CATALOGUE).await;
    assert!(json.contains(LAYOUT_CATALOGUE_DRAG_MIME));
    assert!(json.contains("\"draggable\":true"));
    assert!(json.contains("layout-catalogue.page"));
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_catalogue_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_CATALOGUE_ID);
    assert_eq!(definition.body_key.as_deref(), Some(LAYOUT_PLAY_BODY_CATALOGUE));
}
