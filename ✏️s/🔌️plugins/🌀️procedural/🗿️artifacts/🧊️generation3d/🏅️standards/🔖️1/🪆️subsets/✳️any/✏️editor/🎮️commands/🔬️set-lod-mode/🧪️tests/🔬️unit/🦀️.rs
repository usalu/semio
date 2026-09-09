use super::*;
use crate::editor::generation3d::testkit::{app, dispatch};
use crate::editor::generation3d::Generation3dCommand;

#[semio_framework_async_macros::async_test]
async fn set_lod_mode_is_a_view_action_with_no_artifact_mutations() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app().await;
    let before = app.snapshot().expect("snapshot");
    dispatch(&mut app, Generation3dCommand::SetLodMode(SetLodMode { value: "wireframe".into() })).await;
    assert_eq!(app.snapshot().expect("snapshot"), before, "setLodMode must not mutate the document");
}
