use super::*;
use crate::editor::flow::unit_tests::context::{dispatch, flow_app, settle};
use crate::editor::flow::FlowCommand;

#[semio_framework_async_macros::async_test]
async fn toggle_extension_and_run_action_reorganizes_fixture() {
    let mut app = flow_app().await;
    let before_snapshot = app.snapshot().expect("snapshot").to_host_snapshot();
    let before = before_snapshot.widgets.len();
    before_snapshot.retire_cold();
    let ignored = dispatch(&mut app, FlowCommand::RunExtensionAction(crate::editor::flow::commands::run_extension_action::RunExtensionAction { action_id: "flow.extension.reorganize".into() })).await;
    assert!(ignored.mutations.is_empty(), "disabled automation action must be a no-operation");
    settle(&mut app).await;
    dispatch(&mut app, FlowCommand::ToggleExtension(ToggleExtension { id: "auto-layout".into(), enabled: true })).await;
    settle(&mut app).await;
    dispatch(&mut app, FlowCommand::RunExtensionAction(crate::editor::flow::commands::run_extension_action::RunExtensionAction { action_id: "flow.extension.reorganize".into() })).await;
    settle(&mut app).await;
    let after_snapshot = app.snapshot().expect("snapshot").to_host_snapshot();
    assert_eq!(after_snapshot.widgets.len(), before, "reorganize keeps every widget");
    after_snapshot.retire_cold();
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_extension_action_id_is_a_no_operation() {
    let mut app = flow_app().await;
    let result = dispatch(&mut app, FlowCommand::RunExtensionAction(crate::editor::flow::commands::run_extension_action::RunExtensionAction { action_id: "third.party.nope".into() })).await;
    assert!(result.mutations.is_empty());
    settle(&mut app).await;
}
