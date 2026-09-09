use crate::editor::lowpoly::testkit::{app, render};

#[semio_framework_async_macros::async_test]
async fn renders_uv_canvas() {
    let mut a = app().await;
    assert!(render(&mut a, super::LOWPOLY_PLAY_BODY_UV).await.contains("canvas-2d"));
}
