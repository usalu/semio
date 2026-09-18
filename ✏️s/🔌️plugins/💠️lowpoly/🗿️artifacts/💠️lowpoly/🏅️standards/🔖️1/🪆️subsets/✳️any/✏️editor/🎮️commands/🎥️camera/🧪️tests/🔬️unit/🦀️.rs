use crate::editor::lowpoly::unit_tests::context::{app, committed_edits, dispatch};
use crate::editor::lowpoly::LowpolyCommand;

#[semio_framework_async_macros::async_test]
async fn set_camera_updates_config() {
    let mut a = app().await;
    let edits_before = committed_edits(&mut a).await;
    dispatch(&mut a, LowpolyCommand::SetCamera(super::set_camera::SetCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 })).await;
    // 🎥️ A second identical camera settles just as cleanly and, being window config, never lands in the document.
    dispatch(&mut a, LowpolyCommand::SetCamera(super::set_camera::SetCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 })).await;
    assert_eq!(committed_edits(&mut a).await, edits_before, "setCamera is view state, not a document edit");
}
