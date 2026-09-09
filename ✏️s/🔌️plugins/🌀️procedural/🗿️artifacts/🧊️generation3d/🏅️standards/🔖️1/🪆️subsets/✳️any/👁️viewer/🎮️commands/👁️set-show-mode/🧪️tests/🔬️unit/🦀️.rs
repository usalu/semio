use super::*;
use crate::viewer::generation3d::testkit::{app, dispatch};
use crate::viewer::generation3d::Generation3dViewCommand;

/// 👁️ Dispatches through the REAL interactive-job pipeline — an action that is not `Migrated`, or
/// whose bounded reducer proof is missing, faults here instead of silently doing nothing.
#[semio_framework_async_macros::async_test]
async fn set_show_mode_dispatches_live_and_leaves_the_document_untouched() {
    let _serial = crate::viewer::generation3d::testkit::lock();
    let mut app = app().await;
    let before = app.snapshot().expect("snapshot");
    let command = Generation3dViewCommand::SetShowMode(SetShowMode { value: "wireframe".into() });
    assert_eq!(command.command_id(), "setShowMode");
    dispatch(&mut app, command).await;
    assert_eq!(app.snapshot().expect("snapshot"), before, "a viewer command must never mutate the document");
}
