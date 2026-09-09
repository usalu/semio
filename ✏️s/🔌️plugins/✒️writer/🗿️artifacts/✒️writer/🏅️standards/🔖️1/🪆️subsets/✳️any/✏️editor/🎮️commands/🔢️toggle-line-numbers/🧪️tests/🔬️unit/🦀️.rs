use super::ToggleLineNumbers;
use crate::editor::writer::testkit::new_app;
use crate::editor::writer::WriterCommand;

#[semio_framework_async_macros::async_test]
async fn view_action_emits_no_operations() {
    let mut app = new_app().await;
    let result = app.dispatch_typed(WriterCommand::ToggleLineNumbers(ToggleLineNumbers {}), &semio_framework_plugin::testkit::meta("local")).await.expect("toggle");
    assert!(result.mutations.is_empty());
}
