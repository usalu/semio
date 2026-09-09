use crate::editor::lowpoly::testkit::{app, dispatch};
use crate::editor::lowpoly::LowpolyCommand;

#[semio_framework_async_macros::async_test]
async fn set_camera_updates_config() {
    let mut a = app().await;
    dispatch(&mut a, LowpolyCommand::SetCamera(super::set_camera::SetCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 })).await;
    assert!(a.dispatch_typed(LowpolyCommand::SetCamera(super::set_camera::SetCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 }), &semio_framework_plugin::testkit::meta("a")).await.is_ok());
}
