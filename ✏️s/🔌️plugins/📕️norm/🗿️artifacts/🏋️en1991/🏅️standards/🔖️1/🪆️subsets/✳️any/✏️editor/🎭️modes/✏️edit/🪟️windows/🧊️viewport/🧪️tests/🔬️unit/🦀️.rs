use crate::modes::edit::windows::viewport::*;
use crate::unit_tests::context;

#[semio_framework_async_macros::async_test]
async fn definition_declares_this_windows_body_key() {
    assert_eq!(definition().body_key, BODY_VIEWPORT);
    assert_eq!(definition().id, WINDOW_VIEWPORT);
    assert_eq!(definition().surface_kind, semio_framework_plugin::SurfaceKind::World3d);
}

#[semio_framework_async_macros::async_test]
async fn renders_a_world_surface_without_json_text() {
    let mut app = context::app_with_registry().await;
    let rendered = context::render(&mut app, BODY_VIEWPORT).await;
    assert!(rendered.contains("world-3d") || rendered.contains("World3d") || rendered.contains("world3d"), "the viewport body must render the world surface: {rendered}");
    assert!(!rendered.contains("\"windFaces\""), "no document JSON in the viewport");
    context::close(&mut app);
}
