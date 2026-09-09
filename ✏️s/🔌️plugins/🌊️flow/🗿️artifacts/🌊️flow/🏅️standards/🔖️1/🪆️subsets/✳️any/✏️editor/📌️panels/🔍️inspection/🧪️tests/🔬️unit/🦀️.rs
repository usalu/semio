use super::*;
use crate::editor::flow::testkit::{flow_app, render as render_body};
use crate::editor::flow::FLOW_PLAY_BODY_INSPECTOR as BODY_INSPECTOR;

#[semio_framework_async_macros::async_test]
async fn empty_inspector_no_longer_shows_canvas_settings() {
    let mut app = flow_app().await;
    let json = render_body(&mut app, BODY_INSPECTOR).await;
    assert!(!json.contains("flow-play-inspector.lod-mode"));
    assert!(json.contains("flow-play-inspector.empty"));
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
    assert_eq!(definition.body_key.as_deref(), Some(FLOW_PLAY_BODY_INSPECTOR));
}
