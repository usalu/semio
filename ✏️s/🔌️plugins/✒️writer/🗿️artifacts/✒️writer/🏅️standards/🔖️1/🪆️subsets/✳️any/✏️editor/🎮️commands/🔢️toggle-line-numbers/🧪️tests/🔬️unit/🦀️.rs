use super::ToggleLineNumbers;
use crate::editor::writer::unit_tests::context::new_app;
use crate::editor::writer::WriterCommand;

#[semio_framework_async_macros::async_test]
async fn view_action_emits_no_operations() {
    let mut app = new_app().await;
    let result = crate::editor::writer::unit_tests::context::dispatch(&mut app, WriterCommand::ToggleLineNumbers(ToggleLineNumbers {})).await;
    assert!(result.mutations.is_empty());
}
