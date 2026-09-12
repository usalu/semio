use super::*;
use crate::editor::generation2d::commands::enter_generate;
use crate::editor::generation2d::unit_tests::context::{app, close, dispatch, snapshot_read};
use crate::editor::generation2d::Generation2dCommand;

#[semio_framework_async_macros::async_test]
async fn add_generation_records_an_undoable_generation_operation() {
    let mut app = app().await;
    let before = snapshot_read(&app).generation.generations.len();
    dispatch(&mut app, Generation2dCommand::AddGeneration(AddGeneration {})).await;
    let after = snapshot_read(&app).generation.generations.len();
    close(app);
    assert_eq!(after, before + 1);
}

#[semio_framework_async_macros::async_test]
async fn generate_is_a_view_action_with_no_artifact_mutations() {
    let mut app = app().await;
    let before = snapshot_read(&app);
    dispatch(&mut app, Generation2dCommand::Generate(enter_generate::Generate {})).await;
    let after = snapshot_read(&app);
    let unchanged = after == before;
    close(app);
    assert!(unchanged, "generate must not mutate the document");
}
