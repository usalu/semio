use super::*;
use crate::editor::flow::unit_tests::context::{dispatch, flow_app_closing, flow_app_with_registry, render, select_graph, settle};
use crate::editor::flow::FlowCommand;
use semio_framework_plugin::artifact_app_laws::{meta, settle_registered_typed_operation};
use semio_framework_plugin::PluginApp;

/// 🎯️ The batched `DeleteSelection` sub-op must clear the node selection (visible on the rendered
/// scene) while leaving the widget count intact when nothing resolves — the behavior that
/// distinguishes it from the top-level `FlowCommand::DeleteSelection`.
#[semio_framework_async_macros::async_test]
async fn batched_delete_selection_clears_the_node_selection_on_the_scene() {
    let mut app = flow_app_with_registry().await;
    select_graph(&mut app, &["slider"], &[]).await;
    dispatch(&mut app, FlowCommand::NodeGraphEdit(NodeGraphEdit { operations: vec![FlowNodeGraphEditOp::DeleteSelection] })).await;
    settle(&mut app).await;
    assert!(!app.snapshot().expect("snapshot").to_host_snapshot().widgets.iter().any(|widget| crate::schema::widget_id(widget) == "slider"), "batched delete removes the picked widget");
    let _ = render(&mut app, crate::editor::flow::FLOW_PLAY_BODY_MAIN).await;
}

#[semio_framework_async_macros::async_test]
async fn spotlight_commit_shares_the_node_graph_edit_vocabulary() {
    use crate::editor::flow::commands::spotlight_commit;
    let mut app = flow_app_with_registry().await;
    let result = dispatch(
        &mut app,
        FlowCommand::SpotlightCommit(spotlight_commit::SpotlightCommit {
            operations: vec![spotlight_commit::FlowNodeGraphEditOp::Connect { source_node_id: "slider".into(), source_port_id: "number".into(), target_node_id: "add".into(), target_port_id: "b".into() }],
        }),
    )
    .await;
    assert!(result.mutations.is_empty(), "retained Spotlight admission does not publish synchronously");
    settle(&mut app).await;
    let live = app.snapshot().expect("snapshot after Spotlight publication").to_host_snapshot();
    assert!(live.synapses.iter().any(|edge| edge.from == "slider" && edge.from_port == "number" && edge.to == "add" && edge.to_port == "b"), "Spotlight and nodeGraphEdit share the exact valid connect vocabulary");
    live.retire_cold();
}

#[semio_framework_async_macros::async_test]
async fn renderer_operation_rows_decode_through_one_closed_vocabulary() {
    let host_snapshot = semio_framework_artifact_flow_flow::FlowHostSnapshot::default();
    let host_snapshot_json = dsl::json::to_json_string(&host_snapshot);
    host_snapshot.retire_cold();
    let args = dsl::DslValue::from(serde_json::json!({
        "operations": [
            { "operation": "setHostSnapshot", "hostSnapshotJson": host_snapshot_json.clone() },
            { "operation": "deleteSelection" },
            { "operation": "connect", "sourceNodeId": "slider", "sourcePortId": "number", "targetNodeId": "add", "targetPortId": "a" },
            { "operation": "disconnect", "synapseId": "s1" },
            { "operation": "move", "nodeId": "add", "x": 284.0, "y": 48.0 }
        ]
    }));
    assert_eq!(
        operations_from_action(&args).expect("the renderer's current operation vocabulary"),
        vec![
            FlowNodeGraphEditOp::SetHostSnapshot { host_snapshot_json },
            FlowNodeGraphEditOp::DeleteSelection,
            FlowNodeGraphEditOp::Connect { source_node_id: "slider".into(), source_port_id: "number".into(), target_node_id: "add".into(), target_port_id: "a".into() },
            FlowNodeGraphEditOp::Disconnect { synapse_id: "s1".into() },
            FlowNodeGraphEditOp::Move { node_id: "add".into(), x: 284.0, y: 48.0 },
        ]
    );
    let malformed = dsl::DslValue::from(serde_json::json!({
        "operations": [
            { "operation": "move", "nodeId": "add", "x": 284.0, "y": 48.0 },
            { "operation": "move", "nodeId": "add", "x": 568.0 }
        ]
    }));
    assert!(operations_from_action(&malformed).is_err(), "one malformed row must refuse the complete operation array");
}

#[semio_framework_async_macros::async_test]
async fn operation_parser_refuses_beyond_the_retained_route_row_and_wire_authorities() {
    let too_many = dsl::DslValue::from(serde_json::json!({
        "operations": (0..=crate::editor::flow::FLOW_STORE_MAX_MUTATION_ITEMS).map(|_| serde_json::json!({ "operation": "deleteSelection" })).collect::<Vec<_>>()
    }));
    assert!(operations_from_action(&too_many).is_err(), "the parser must refuse before walking row 257");

    let oversized = dsl::DslValue::from(serde_json::json!({
        "operations": [{ "operation": "move", "nodeId": "x".repeat(crate::editor::flow::FLOW_GRAPH_OPERATION_RAW_BYTES), "x": 1.0, "y": 2.0 }]
    }));
    assert!(operations_from_action(&oversized).is_err(), "the parser must share the retained route's 16 KiB wire authority");
}

#[semio_framework_async_macros::async_test]
async fn node_graph_move_wire_publishes_the_requested_widget_layout() {
    let mut app = flow_app_closing().await;
    let initial = app.snapshot().expect("snapshot before nodeGraphEdit move").to_host_snapshot();
    assert!(initial.widgets.iter().any(|widget| crate::schema::widget_id(widget) == "add"), "the real starter graph must contain the renderer's move target");
    let initial_position = initial.layout.get("add").map_or((0.0, 0.0), |layout| (layout.x, layout.y));
    initial.retire_cold();
    assert_eq!(initial_position, (0.0, 0.0), "the move law must observe the target's actual starting position");
    let args = dsl::DslValue::from(serde_json::json!({
        "operations": [{ "operation": "move", "nodeId": "add", "x": 284.0, "y": 48.0 }]
    }));
    app.handle_action("nodeGraphEdit", Some(&args), &meta("flow-node-graph-move")).await.expect("nodeGraphEdit move admission");
    settle_registered_typed_operation(&mut *app, 1).await.expect("nodeGraphEdit move publication");
    let live = app.snapshot().expect("snapshot after nodeGraphEdit move").to_host_snapshot();
    let moved = live.layout.iter().find(|(id, _)| id.as_str() == "add").map(|(_, layout)| (layout.x, layout.y));
    live.retire_cold();
    assert_eq!(moved, Some((284.0, 48.0)), "the renderer's exact move row must survive strict wire parsing and reach the Flow host layout");
}

#[semio_framework_async_macros::async_test]
async fn node_graph_edit_rejects_an_unknown_operation_instead_of_dropping_it() {
    let mut app = flow_app_closing().await;
    let args = dsl::DslValue::from(serde_json::json!({
        "operations": [
            { "operation": "move", "nodeId": "add", "x": 284.0, "y": 48.0 },
            { "operation": "teleport", "nodeId": "add", "x": 568.0, "y": 96.0 }
        ]
    }));
    let result = app.handle_action("nodeGraphEdit", Some(&args), &meta("flow-node-graph-invalid")).await;
    assert!(result.is_err(), "one malformed operation must reject the whole bounded edit before its valid prefix is admitted");
    assert!(!app.has_pending_typed_operations(), "atomic refusal cannot leave a retained operation to publish the valid prefix");
    let live = app.snapshot().expect("snapshot after refused nodeGraphEdit batch").to_host_snapshot();
    assert!(live.layout.get("add").is_none(), "atomic refusal cannot move the valid prefix's target");
    live.retire_cold();
}
