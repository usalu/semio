use super::*;
use crate::editor::generation3d::commands::select_generation;
use crate::editor::generation3d::unit_tests::context::{app, dispatch};
use crate::editor::generation3d::Generation3dCommand;
use semio_framework_plugin::artifact_app_laws::assert_undo_redo_round_trip;
use crate::editor::generation3d::unit_tests::context;

#[semio_framework_async_macros::async_test]
async fn add_generation_records_an_undoable_generation_operation() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    assert_undo_redo_round_trip(&mut app, Generation3dCommand::AddGeneration(AddGeneration {}), |app| context::snapshot(&app).generation.generations.len(), 0, 1).await;
}

#[semio_framework_async_macros::async_test]
async fn generate_mode_renders_surfaces() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    assert!(crate::editor::generation3d::unit_tests::context::render(&mut app, crate::editor::generation3d::modes::generate::windows::generations::GENERATION_3D_PLAY_BODY_GENERATIONS).await.contains("addGeneration"));
}

#[semio_framework_async_macros::async_test]
async fn select_generation_does_not_mutate_the_document() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    dispatch(&mut app, Generation3dCommand::AddGeneration(AddGeneration {})).await;
    let before = context::snapshot(&app);
    let generation_id = before.generation.generations.first().expect("generation").id.clone();
    dispatch(&mut app, Generation3dCommand::SelectGeneration(select_generation::SelectGeneration { id: generation_id })).await;
    assert_eq!(context::snapshot(&app), before);
}
