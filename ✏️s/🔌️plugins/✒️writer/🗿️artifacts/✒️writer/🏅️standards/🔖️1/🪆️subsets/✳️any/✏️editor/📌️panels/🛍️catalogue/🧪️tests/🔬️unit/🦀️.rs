use super::*;
use crate::editor::writer::testkit::{new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_catalogue_panel() {
    let mut app = new_app().await;
    assert!(render_body(&mut app, WRITER_PLAY_BODY_CATALOGUE).await.contains("jack"));
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_catalogue_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_CATALOGUE_ID);
    assert_eq!(definition.body_key.as_deref(), Some(WRITER_PLAY_BODY_CATALOGUE));
}
