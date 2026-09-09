use super::*;
use crate::editor::generation2d::testkit::{app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn document_lists_widgets() {
    let mut app = app().await;
    let rendered = render_body(&mut app, GENERATION2D_PLAY_BODY_DOCUMENT).await;
    let fixture_widgets: Vec<String> = app.snapshot().expect("snapshot").fixture.widgets.iter().map(|widget| widget_id(widget).to_string()).collect();
    let first = fixture_widgets.first().expect("default fixture has at least one widget");
    assert!(rendered.contains(first), "document tree missing widget id {first}: {rendered}");
}

#[test]
fn definition_binds_the_framework_document_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    assert_eq!(definition.body_key.as_deref(), Some(GENERATION2D_PLAY_BODY_DOCUMENT));
}
