use super::ToggleLineNumbers;
use crate::editor::writer::unit_tests::context::new_app;
use crate::editor::writer::WriterCommand;

#[semio_framework_async_macros::async_test]
async fn view_action_emits_no_operations() {
    let mut app = new_app().await;
    let result = app.dispatch_typed(WriterCommand::ToggleLineNumbers(ToggleLineNumbers {}), &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("toggle");
    assert!(result.mutations.is_empty());
}
