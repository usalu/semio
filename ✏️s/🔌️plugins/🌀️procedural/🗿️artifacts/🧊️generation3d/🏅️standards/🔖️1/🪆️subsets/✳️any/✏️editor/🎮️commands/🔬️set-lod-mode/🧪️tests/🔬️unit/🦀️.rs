use super::*;
use crate::editor::generation3d::unit_tests::context::{app, dispatch};
use crate::editor::generation3d::Generation3dCommand;
use crate::editor::generation3d::unit_tests::context;

#[semio_framework_async_macros::async_test]
async fn set_lod_mode_is_a_view_action_with_no_artifact_mutations() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    let before = context::snapshot(&app);
    dispatch(&mut app, Generation3dCommand::SetLodMode(SetLodMode { value: "wireframe".into() })).await;
    assert_eq!(context::snapshot(&app), before, "setLodMode must not mutate the document");
}
