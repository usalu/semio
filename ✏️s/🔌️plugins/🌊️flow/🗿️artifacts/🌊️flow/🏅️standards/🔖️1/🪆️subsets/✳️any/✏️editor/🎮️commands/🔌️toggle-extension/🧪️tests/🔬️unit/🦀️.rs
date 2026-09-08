
use super::*;
use crate::editor::flow::FlowCommand;
use crate::editor::flow::testkit::{dispatch, flow_app};

#[semio_framework_async_macros::async_test]
async fn toggle_extension_and_run_action_reorganizes_fixture() {
    let mut app = flow_app().await;
    let before = app.snapshot().expect("snapshot").to_fixture().widgets.len();
    let ignored = dispatch(&mut app, FlowCommand::RunExtensionAction(crate::editor::flow::commands::run_extension_action::RunExtensionAction { action_id: "flow.extension.reorganize".into() })).await;
    assert!(ignored.mutations.is_empty(), "disabled automation action must be a no-operation");
    dispatch(&mut app, FlowCommand::ToggleExtension(ToggleExtension { id: "auto-layout".into(), enabled: true })).await;
    dispatch(&mut app, FlowCommand::RunExtensionAction(crate::editor::flow::commands::run_extension_action::RunExtensionAction { action_id: "flow.extension.reorganize".into() })).await;
    assert_eq!(app.snapshot().expect("snapshot").to_fixture().widgets.len(), before, "reorganize keeps every widget");
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_extension_action_id_is_a_no_operation() {
    let mut app = flow_app().await;
    let result = dispatch(&mut app, FlowCommand::RunExtensionAction(crate::editor::flow::commands::run_extension_action::RunExtensionAction { action_id: "third.party.nope".into() })).await;
    assert!(result.mutations.is_empty());
}
