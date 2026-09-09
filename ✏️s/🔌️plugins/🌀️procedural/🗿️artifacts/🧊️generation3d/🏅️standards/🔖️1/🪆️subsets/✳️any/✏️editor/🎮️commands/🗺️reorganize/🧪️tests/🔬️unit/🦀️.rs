use super::*;
use crate::editor::generation3d::testkit::{app, dispatch};
use crate::editor::generation3d::Generation3dCommand;

#[semio_framework_async_macros::async_test]
async fn set_lod_mode_is_a_view_action_with_no_artifact_mutations_via_reorganize_baseline() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app().await;
    let before = app.snapshot().expect("snapshot").fixture.widgets.len();
    dispatch(&mut app, Generation3dCommand::Reorganize(Reorganize {})).await;
    assert_eq!(app.snapshot().expect("snapshot").fixture.widgets.len(), before);
}
