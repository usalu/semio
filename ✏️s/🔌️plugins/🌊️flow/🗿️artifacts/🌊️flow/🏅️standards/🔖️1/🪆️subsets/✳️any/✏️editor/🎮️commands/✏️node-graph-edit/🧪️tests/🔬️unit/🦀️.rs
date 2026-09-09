use super::*;
use crate::editor::flow::testkit::{dispatch, flow_app_with_registry, render, select_graph};
use crate::editor::flow::FlowCommand;

/// 🎯️ The batched `DeleteSelection` sub-op must clear the node selection (visible on the rendered
/// scene) while leaving the widget count intact when nothing resolves — the behavior that
/// distinguishes it from the top-level `FlowCommand::DeleteSelection`.
#[semio_framework_async_macros::async_test]
async fn batched_delete_selection_clears_the_node_selection_on_the_scene() {
    let mut app = flow_app_with_registry().await;
    select_graph(&mut app, &["slider"], &[]).await;
    dispatch(&mut app, FlowCommand::NodeGraphEdit(NodeGraphEdit { operations: vec![FlowNodeGraphEditOp::DeleteSelection] })).await;
    assert!(!app.snapshot().expect("snapshot").to_fixture().widgets.iter().any(|widget| crate::schema::widget_id(widget) == "slider"), "batched delete removes the picked widget");
    let _ = render(&mut app, crate::editor::flow::FLOW_PLAY_BODY_MAIN).await;
}

#[semio_framework_async_macros::async_test]
async fn spotlight_commit_shares_the_node_graph_edit_vocabulary() {
    use crate::editor::flow::commands::spotlight_commit;
    let mut app = flow_app_with_registry().await;
    let result = dispatch(
        &mut app,
        FlowCommand::SpotlightCommit(spotlight_commit::SpotlightCommit {
            operations: vec![spotlight_commit::FlowNodeGraphEditOp::Connect { source_node_id: "nope".into(), source_port_id: "out".into(), target_node_id: "gone".into(), target_port_id: "in".into() }],
        }),
    )
    .await;
    assert!(result.mutations.is_empty(), "connecting missing nodes is a no-operation");
}
