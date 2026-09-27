use super::*;
use crate::editor::flow::unit_tests::context::{composed_scene, dispatch, flow_app, settle, settle_refusal};
use crate::editor::flow::FlowCommand;

#[semio_framework_async_macros::async_test]
async fn toggle_extension_and_run_action_reorganizes_fixture() {
    let mut app = flow_app().await;
    let before_snapshot = composed_scene(&app).await;
    let before = before_snapshot.widgets.len();
    before_snapshot.retire_cold();
    let refused = dispatch(&mut app, FlowCommand::RunExtensionAction(crate::editor::flow::commands::run_extension_action::RunExtensionAction { action_id: "flow.extension.reorganize".into() })).await;
    assert!(refused.mutations.is_empty(), "a disabled automation action publishes nothing");
    let refusal = settle_refusal(&mut app).await;
    assert!(refusal.contains("flow.extension-disabled"), "a disabled automation action is refused by name: {refusal}");
    dispatch(&mut app, FlowCommand::ToggleExtension(ToggleExtension { id: "auto-layout".into(), enabled: true })).await;
    settle(&mut app).await;
    dispatch(&mut app, FlowCommand::RunExtensionAction(crate::editor::flow::commands::run_extension_action::RunExtensionAction { action_id: "flow.extension.reorganize".into() })).await;
    settle(&mut app).await;
    let after_snapshot = composed_scene(&app).await;
    assert_eq!(after_snapshot.widgets.len(), before, "reorganize keeps every widget");
    after_snapshot.retire_cold();
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_extension_action_id_is_refused_by_name() {
    let mut app = flow_app().await;
    let result = dispatch(&mut app, FlowCommand::RunExtensionAction(crate::editor::flow::commands::run_extension_action::RunExtensionAction { action_id: "third.party.nope".into() })).await;
    assert!(result.mutations.is_empty());
    let refusal = settle_refusal(&mut app).await;
    assert!(refusal.contains("flow.extension-action-unknown") && refusal.contains("third.party.nope"), "{refusal}");
}
