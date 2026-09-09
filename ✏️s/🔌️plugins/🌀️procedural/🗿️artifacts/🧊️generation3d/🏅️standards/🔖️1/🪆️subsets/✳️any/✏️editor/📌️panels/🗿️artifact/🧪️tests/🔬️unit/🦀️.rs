use super::*;
use crate::editor::generation3d::testkit::{app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn document_lists_widgets() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app().await;
    let rendered = render_body(&mut app, GENERATION_3D_PLAY_BODY_DOCUMENT).await;
    let fixture_widgets: Vec<String> = app.snapshot().expect("snapshot").fixture.widgets.iter().map(|widget| widget_id(widget).to_string()).collect();
    let first = fixture_widgets.first().expect("default fixture has at least one widget");
    assert!(rendered.contains(first), "document tree missing widget id {first}: {rendered}");
}
