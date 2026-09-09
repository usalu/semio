use super::*;
use crate::editor::animate::testkit::{presentation_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_catalogue_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_CATALOGUE_ID);
    assert_eq!(definition.body_key.as_deref(), Some(PRESENTATION_PLAY_BODY_CATALOGUE));
}

#[semio_framework_async_macros::async_test]
async fn catalogue_lists_templates() {
    let mut app = presentation_app().await;
    assert!(render_body(&mut app, PRESENTATION_PLAY_BODY_CATALOGUE).await.contains("animate.presentation.play.catalogue.templates"));
}
