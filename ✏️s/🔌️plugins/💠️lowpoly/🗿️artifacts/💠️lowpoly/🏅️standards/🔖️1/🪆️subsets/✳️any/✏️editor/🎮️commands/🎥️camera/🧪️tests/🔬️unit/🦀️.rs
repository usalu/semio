extern crate semio_framework_os_kernel as protocol;
#[path = "../../../../🧪️tests/🧰️support/🦀️.rs"]
mod context;
use crate::context::{app, committed_edits, dispatch};
use semio_s_artifact_lowpoly_lowpoly_editor::LowpolyCommand;

#[semio_framework_async_macros::async_test]
async fn set_camera_updates_config() {
    let mut a = app().await;
    let edits_before = committed_edits(&mut a).await;
    dispatch(&mut a, LowpolyCommand::SetCamera(semio_s_artifact_lowpoly_lowpoly_commands::camera::set_camera::SetCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 })).await;
    dispatch(&mut a, LowpolyCommand::SetCamera(semio_s_artifact_lowpoly_lowpoly_commands::camera::set_camera::SetCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 })).await;
    assert_eq!(committed_edits(&mut a).await, edits_before, "setCamera is view state, not a document edit");
}
