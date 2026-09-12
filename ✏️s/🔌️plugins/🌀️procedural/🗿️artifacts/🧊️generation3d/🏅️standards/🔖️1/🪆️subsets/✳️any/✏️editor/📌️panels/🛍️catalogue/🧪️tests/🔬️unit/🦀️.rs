use super::*;
use crate::editor::generation3d::unit_tests::context::{app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn generation3d_labels_resolve_native_english_by_default() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    let json = render_body(&mut app, GENERATION_3D_PLAY_BODY_CATALOGUE).await;
    assert!(json.contains("\"Widgets\""));
    assert!(!json.contains("Elemente"));
}
