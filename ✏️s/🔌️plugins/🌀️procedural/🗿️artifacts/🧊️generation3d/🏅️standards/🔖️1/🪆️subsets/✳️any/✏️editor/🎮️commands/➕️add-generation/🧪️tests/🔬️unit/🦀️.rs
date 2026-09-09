use super::*;
use crate::editor::generation3d::commands::select_generation;
use crate::editor::generation3d::testkit::{app, dispatch};
use crate::editor::generation3d::Generation3dCommand;
use semio_framework_plugin::testkit::assert_undo_redo_round_trip;
use crate::editor::generation3d::testkit;

#[semio_framework_async_macros::async_test]
async fn add_generation_records_an_undoable_generation_operation() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app().await;
    assert_undo_redo_round_trip(&mut app, Generation3dCommand::AddGeneration(AddGeneration {}), |app| testkit::snapshot(&app).generation.generations.len(), 0, 1).await;
}

#[semio_framework_async_macros::async_test]
async fn generate_mode_renders_surfaces() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app().await;
    assert!(crate::editor::generation3d::testkit::render(&mut app, crate::editor::generation3d::modes::generate::windows::generations::GENERATION_3D_PLAY_BODY_GENERATIONS).await.contains("addGeneration"));
}

#[semio_framework_async_macros::async_test]
async fn select_generation_does_not_mutate_the_document() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app().await;
    dispatch(&mut app, Generation3dCommand::AddGeneration(AddGeneration {})).await;
    let before = testkit::snapshot(&app);
    let generation_id = before.generation.generations.first().expect("generation").id.clone();
    dispatch(&mut app, Generation3dCommand::SelectGeneration(select_generation::SelectGeneration { id: generation_id })).await;
    assert_eq!(testkit::snapshot(&app), before);
}
