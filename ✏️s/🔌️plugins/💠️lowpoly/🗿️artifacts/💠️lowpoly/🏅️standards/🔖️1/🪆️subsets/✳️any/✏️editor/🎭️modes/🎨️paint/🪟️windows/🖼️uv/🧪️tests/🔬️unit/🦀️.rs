extern crate semio_framework_os_kernel as protocol;
#[path = "../../../../../../🧪️tests/🧰️support/🦀️.rs"]
mod context;
use crate::context::{app, render};

#[semio_framework_async_macros::async_test]
async fn renders_uv_canvas() {
    let mut a = app().await;
    assert!(render(&mut a, semio_s_artifact_lowpoly_lowpoly_modes::editor::lowpoly::modes::paint::windows::uv::LOWPOLY_PLAY_BODY_UV).await.contains("canvas-2d"));
}
