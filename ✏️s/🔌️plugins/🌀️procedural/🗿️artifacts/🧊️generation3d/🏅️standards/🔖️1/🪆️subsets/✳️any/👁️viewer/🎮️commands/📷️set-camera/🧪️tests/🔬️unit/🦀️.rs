use super::*;
use crate::viewer::generation3d::unit_tests::context::{app, dispatch};
use crate::viewer::generation3d::Generation3dViewCommand;
use crate::viewer::generation3d::unit_tests::context;

/// 👁️ Dispatches through the REAL interactive-job pipeline — an action that is not `Migrated`, or
/// whose bounded reducer proof is missing, faults here instead of silently doing nothing.
#[semio_framework_async_macros::async_test]
async fn set_camera_dispatches_live_and_leaves_the_document_untouched() {
    let _serial = crate::viewer::generation3d::unit_tests::context::lock();
    let mut app = app().await;
    let before = context::snapshot(&app);
    let command = Generation3dViewCommand::SetCamera(SetCamera { camera: Generation3dViewCamera { position: [9.0, 8.0, 7.0], target: [0.0, 0.0, 0.0], fov: 33.0 } });
    assert_eq!(command.command_id(), "setCamera");
    dispatch(&mut app, command).await;
    assert_eq!(context::snapshot(&app), before, "a viewer command must never mutate the document");
}
