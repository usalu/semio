use super::*;
use crate::editor::generation2d::commands::enter_generate;
use crate::editor::generation2d::testkit::{app, dispatch};
use crate::editor::generation2d::Generation2dCommand;

#[semio_framework_async_macros::async_test]
async fn add_generation_records_an_undoable_generation_operation() {
    let mut app = app().await;
    let before = app.snapshot().expect("snapshot").generation.generations.len();
    dispatch(&mut app, Generation2dCommand::AddGeneration(AddGeneration {})).await;
    assert_eq!(app.snapshot().expect("snapshot").generation.generations.len(), before + 1);
}

#[semio_framework_async_macros::async_test]
async fn generate_is_a_view_action_with_no_artifact_mutations() {
    let mut app = app().await;
    let before = app.snapshot().expect("snapshot");
    dispatch(&mut app, Generation2dCommand::Generate(enter_generate::Generate {})).await;
    assert_eq!(app.snapshot().expect("snapshot"), before, "generate must not mutate the document");
}
