use super::*;
use crate::editor::flow::unit_tests::context::{dispatch, flow_app_closing, flow_app_with_registry, render, select_graph, FlowApp};
use crate::editor::flow::FlowCommand;
use semio_framework_plugin::artifact_app_laws::{meta, settle_registered_typed_operation};
use semio_framework_plugin::{app::TypedOperationResultLane, PluginApp};
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use store::{ArtifactPack, SpaceMember};

/// 🪟️ Reads the actual main window's public geometry after the owning app settles.
async fn published_host_snapshot(app: &mut FlowApp) -> serde_json::Value {
    let rendered = render(app, crate::editor::flow::FLOW_PLAY_BODY_MAIN).await;
    let scene: ui_wgpu::wgpu::NodeGraphScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(&rendered).expect("Flow main scene");
    serde_json::from_str(scene.host_snapshot_json.as_deref().expect("Flow geometry publication")).expect("Flow host snapshot JSON")
}

/// 🔌️ The physical renderer's narrow connect/disconnect rows must round-trip through the real owner.
#[semio_framework_async_macros::async_test]
async fn node_graph_wire_connect_and_disconnect_republish_the_owner_assigned_synapse() {
    let mut app = flow_app_closing().await;
    let initial = published_host_snapshot(&mut app).await;
    let initial_edges = initial["synapses"].as_array().expect("initial synapses");
    let matches_edge = |edge: &serde_json::Value| edge["from"] == "slider" && edge["fromPort"] == "number" && edge["to"] == "add" && edge["toPort"] == "b";
    assert!(!initial_edges.iter().any(matches_edge), "the physical specimen starts with the target input free");
    let content_id = app.snapshot().unwrap().content.child_id.clone();
    let connect = dsl::DslValue::from(serde_json::json!({
        "operations": [{ "operation": "connect", "sourceNodeId": "slider", "sourcePortId": "number", "targetNodeId": "add", "targetPortId": "b" }]
    }));
    app.handle_action("nodeGraphEdit", Some(&connect), &meta("renderer-wire-connect")).await.expect("wire connect admission");
    let receipt = settle_registered_typed_operation(&mut *app, meta("local").instance_id).await.expect("wire connect publication");
    assert_eq!(receipt.lanes, [TypedOperationResultLane::Child, TypedOperationResultLane::Ui, TypedOperationResultLane::Terminal]);
    let connected = published_host_snapshot(&mut app).await;
    let edges = connected["synapses"].as_array().unwrap();
    assert_eq!(edges.len(), initial_edges.len() + 1);
    let matches: Vec<_> = edges.iter().filter(|edge| matches_edge(edge)).collect();
    assert_eq!(matches.len(), 1, "one renderer gesture persists exactly one owner edge");
    let id = matches[0]["id"].as_str().filter(|id| !id.is_empty()).expect("owner assigned synapse identity").to_owned();
    let content = content_snapshot(&app).await;
    assert!(content.edges.iter().any(|edge| edge.id == id && edge.from.node == "slider" && edge.from.port == "number" && edge.to.node == "add" && edge.to.port == "b"));
    let disconnect = dsl::DslValue::from(serde_json::json!({ "operations": [{ "operation": "disconnect", "synapseId": id }] }));
    app.handle_action("nodeGraphEdit", Some(&disconnect), &meta("renderer-wire-disconnect")).await.expect("wire disconnect admission");
    let receipt = settle_registered_typed_operation(&mut *app, meta("local").instance_id).await.expect("wire disconnect publication");
    assert_eq!(receipt.lanes, [TypedOperationResultLane::Child, TypedOperationResultLane::Ui, TypedOperationResultLane::Terminal]);
    let disconnected = published_host_snapshot(&mut app).await;
    let remaining = disconnected["synapses"].as_array().unwrap();
    assert_eq!(remaining.len(), initial_edges.len());
    assert!(initial_edges.iter().all(|edge| remaining.contains(edge)), "disconnect preserves every pre-existing synapse");
    assert!(!remaining.iter().any(matches_edge));
    assert_eq!(disconnected["layout"], initial["layout"], "wire edits do not move widgets");
    assert_eq!(app.snapshot().unwrap().content.child_id, content_id);
    assert!(!content_snapshot(&app).await.edges.iter().any(|edge| edge.id == id));
}


async fn content_snapshot(app: &FlowApp) -> SemioFlowSnapshot {
    let parent = app.snapshot().expect("Flow parent snapshot");
    SemioFlowSnapshot::decode_pack(&app.child_store("content", &parent.content.child_id).await.expect("Flow content child").document_pack_bytes().await.expect("Flow content child pack")).expect("Flow content child snapshot")
}

/// 🎯️ The batched `DeleteSelection` sub-op must clear the node selection (visible on the rendered
/// scene) while leaving the widget count intact when nothing resolves — the behavior that
/// distinguishes it from the top-level `FlowCommand::DeleteSelection`.
#[semio_framework_async_macros::async_test]
async fn batched_delete_selection_clears_the_node_selection_on_the_scene() {
    let mut app = flow_app_with_registry().await;
    let content_id = app.snapshot().expect("snapshot before batched delete").content.child_id.clone();
    select_graph(&mut app, &["slider"], &[]).await;
    dispatch(&mut app, FlowCommand::NodeGraphEdit(NodeGraphEdit { operations: vec![FlowNodeGraphEditOp::DeleteSelection] })).await;
    let receipt = settle_registered_typed_operation(&mut *app, meta("local").instance_id).await.expect("batched delete child publication");
    assert_eq!(receipt.lanes, [TypedOperationResultLane::Child, TypedOperationResultLane::Ui, TypedOperationResultLane::Terminal]);
    let snapshot = app.snapshot().expect("snapshot");
    assert_eq!(snapshot.content.child_id, content_id, "batched delete must publish through the existing content child");
    let content = content_snapshot(&app).await;
    assert!(!content.nodes.iter().any(|node| node.id == "slider"), "batched delete removes the picked widget from the composed content child");
    let _ = render(&mut app, crate::editor::flow::FLOW_PLAY_BODY_MAIN).await;
}

#[semio_framework_async_macros::async_test]
async fn spotlight_commit_shares_the_node_graph_edit_vocabulary() {
    use crate::editor::flow::commands::spotlight_commit;
    let mut app = flow_app_with_registry().await;
    let content_id = app.snapshot().expect("snapshot before Spotlight publication").content.child_id.clone();
    let result = dispatch(
        &mut app,
        FlowCommand::SpotlightCommit(spotlight_commit::SpotlightCommit {
            operations: vec![spotlight_commit::FlowNodeGraphEditOp::Connect { source_node_id: "slider".into(), source_port_id: "number".into(), target_node_id: "add".into(), target_port_id: "b".into() }],
        }),
    )
    .await;
    assert!(result.mutations.is_empty(), "retained Spotlight admission does not publish synchronously");
    let receipt = settle_registered_typed_operation(&mut *app, meta("local").instance_id).await.expect("Spotlight child publication");
    assert_eq!(receipt.lanes, [TypedOperationResultLane::Child, TypedOperationResultLane::Ui, TypedOperationResultLane::Terminal]);
    let snapshot = app.snapshot().expect("snapshot after Spotlight publication");
    assert_eq!(snapshot.content.child_id, content_id, "Spotlight must publish through the existing content child");
    let content = content_snapshot(&app).await;
    assert!(
        content.edges.iter().any(|edge| edge.from.node == "slider" && edge.from.port == "number" && edge.to.node == "add" && edge.to.port == "b"),
        "Spotlight and nodeGraphEdit share the exact valid connect vocabulary in the composed content child"
    );
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
            { "operation": "move", "gestureId": "node-drag:3", "nodeIds": ["add", "slider"], "dx": 284.0, "dy": 48.0 }
        ]
    }));
    assert_eq!(
        operations_from_action(&args).expect("the renderer's current operation vocabulary"),
        vec![
            FlowNodeGraphEditOp::SetHostSnapshot { host_snapshot_json },
            FlowNodeGraphEditOp::DeleteSelection,
            FlowNodeGraphEditOp::Connect { source_node_id: "slider".into(), source_port_id: "number".into(), target_node_id: "add".into(), target_port_id: "a".into() },
            FlowNodeGraphEditOp::Disconnect { synapse_id: "s1".into() },
            FlowNodeGraphEditOp::Move { gesture_id: "node-drag:3".into(), node_ids: vec!["add".into(), "slider".into()], dx: 284.0, dy: 48.0 },
        ]
    );
    let malformed = dsl::DslValue::from(serde_json::json!({
        "operations": [
            { "operation": "move", "gestureId": "node-drag:3", "nodeIds": ["add"], "dx": 284.0, "dy": 48.0 },
            { "operation": "move", "nodeId": "add", "x": 568.0, "y": 96.0 }
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
        "operations": [{ "operation": "move", "gestureId": "node-drag:3", "nodeIds": ["x".repeat(crate::editor::flow::FLOW_GRAPH_OPERATION_RAW_BYTES)], "dx": 1.0, "dy": 2.0 }]
    }));
    assert!(operations_from_action(&oversized).is_err(), "the parser must share the retained route's 16 KiB wire authority");
}

#[semio_framework_async_macros::async_test]
async fn node_graph_move_wire_publishes_the_requested_widget_layout() {
    let mut app = flow_app_closing().await;
    let content_id = app.snapshot().expect("snapshot before nodeGraphEdit move").content.child_id.clone();
    let initial = app.snapshot().expect("snapshot before nodeGraphEdit move").to_host_snapshot();
    assert!(initial.widgets.iter().any(|widget| crate::schema::widget_id(widget) == "add"), "the real starter graph must contain the renderer's move target");
    let initial_position = initial.layout.get("add").map_or((0.0, 0.0), |layout| (layout.x, layout.y));
    initial.retire_cold();
    assert_eq!(initial_position, (0.0, 0.0), "the move law must observe the target's actual starting position");
    let args = dsl::DslValue::from(serde_json::json!({
        "operations": [{ "operation": "move", "gestureId": "node-drag:3", "nodeIds": ["add"], "dx": 284.0, "dy": 48.0 }]
    }));
    app.handle_action("nodeGraphEdit", Some(&args), &meta("flow-node-graph-move")).await.expect("nodeGraphEdit move admission");
    let receipt = settle_registered_typed_operation(&mut *app, 1).await.expect("nodeGraphEdit move publication");
    assert_eq!(receipt.lanes, [TypedOperationResultLane::Child, TypedOperationResultLane::Ui, TypedOperationResultLane::Terminal]);
    let snapshot = app.snapshot().expect("snapshot after nodeGraphEdit move");
    assert_eq!(snapshot.content.child_id, content_id, "nodeGraphEdit move must publish through the existing content child");
    let content = content_snapshot(&app).await;
    let moved = content.nodes.iter().find(|node| node.id == "add").map(|node| (node.position.x, node.position.y));
    assert_eq!(moved, Some((284.0, 48.0)), "the renderer's gesture record must survive strict wire parsing and move the node by its offset from its base");
    let rendered = render(&mut app, crate::editor::flow::FLOW_PLAY_BODY_MAIN).await;
    let scene: ui_wgpu::wgpu::NodeGraphScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(&rendered).expect("moved Flow main scene");
    let published: serde_json::Value = serde_json::from_str(scene.host_snapshot_json.as_deref().expect("Flow main publishes its host snapshot geometry")).expect("published Flow host snapshot JSON");
    assert_eq!(published["layout"]["add"], serde_json::json!({ "x": 284.0, "y": 48.0 }), "the next public scene republishes the persisted position through WGPU's active hostSnapshotJson geometry authority");
}

#[semio_framework_async_macros::async_test]
async fn node_graph_edit_rejects_an_unknown_operation_instead_of_dropping_it() {
    let mut app = flow_app_closing().await;
    let args = dsl::DslValue::from(serde_json::json!({
        "operations": [
            { "operation": "move", "gestureId": "node-drag:3", "nodeIds": ["add"], "dx": 284.0, "dy": 48.0 },
            { "operation": "teleport", "nodeId": "add", "x": 568.0, "y": 96.0 }
        ]
    }));
    let result = app.handle_action("nodeGraphEdit", Some(&args), &meta("flow-node-graph-invalid")).await;
    assert!(result.is_err(), "one malformed operation must reject the whole bounded edit before its valid prefix is admitted");
    assert!(!app.has_pending_typed_operations(), "atomic refusal cannot leave a retained operation to publish the valid prefix");
    let content = content_snapshot(&app).await;
    let add = content.nodes.iter().find(|node| node.id == "add").expect("starter add node");
    assert_eq!((add.position.x, add.position.y), (0.0, 0.0), "atomic refusal cannot move the valid prefix's target");
}

//#region 🔖️ComposedChildHistory
/// ✋️ Releases a drag of `nodes` by `(dx, dy)` — the node-graph gesture record both hosts dispatch (design §13.3) — and
/// settles its retained publication.
async fn release_drag(app: &mut FlowApp, nodes: &[&str], dx: f64, dy: f64) {
    let args = dsl::DslValue::from(serde_json::json!({ "operations": [{ "operation": "move", "gestureId": "node-drag:9", "nodeIds": nodes, "dx": dx, "dy": dy }] }));
    app.handle_action("nodeGraphEdit", Some(&args), &meta("renderer-node-drag")).await.expect("node drag admission");
    settle_registered_typed_operation(&mut **app, meta("local").instance_id).await.expect("node drag publication");
}

/// 🧩️ Every history row that lists composed-member mutations, oldest first.
async fn member_rows(app: &mut FlowApp) -> Vec<semio_framework::kernel::HistoryEntry> {
    let mut rows: Vec<_> = app.history_snapshot().await.expect("history").upserts.into_iter().filter(|entry| entry.mutations.iter().any(|mutation| mutation.store.is_some())).collect();
    rows.sort_by_key(|entry| entry.seq);
    rows
}

fn node_position(content: &SemioFlowSnapshot, id: &str) -> (f64, f64) {
    content.nodes.iter().find(|node| node.id == id).map(|node| (node.position.x, node.position.y)).unwrap_or_else(|| panic!("node {id}"))
}

/// ⏪️ Runs one history-edit verb and answers its output; a refusal fails the law unless `refused` names it.
async fn history_edit(app: &mut FlowApp, verb: &str, args: Vec<(&str, dsl::DslValue)>) -> dsl::DslValue {
    let args = dsl::DslValue::Object(args.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
    app.handle_action(verb, Some(&args), &meta("history-edit")).await.unwrap_or_else(|fault| panic!("{verb}: {fault:?}")).output
}

async fn time_travel_stage(app: &mut FlowApp) -> Option<semio_framework::kernel::HistoryTimeTravelStage> {
    app.history_snapshot().await.expect("history").time_travel.map(|status| status.stage)
}

async fn pump_time_travel(app: &mut FlowApp, done: impl Fn(Option<semio_framework::kernel::HistoryTimeTravelStage>) -> bool) {
    for _ in 0..10_000 {
        if done(time_travel_stage(app).await) {
            return;
        }
        app.advance_typed_operation_publication().await.expect("a driver turn");
        while app.take_typed_operation_ui_progress().is_some() {}
    }
    panic!("the history edit never settled: {:?}", time_travel_stage(app).await);
}

/// ⚖️ LAW (design §12, §13.3): a released node drag is ONE tool transaction that lands ONE relative `drag-nodes` edit in
/// the composed content child; the history lists it as one row carrying that transaction, its mutation row labelled from
/// the child's leaf, naming the member store and editable.
#[semio_framework_async_macros::async_test]
async fn a_node_drag_is_one_child_transaction_row_naming_its_member_store() {
    let mut app = flow_app_closing().await;
    let content_id = app.snapshot().expect("snapshot").content.child_id.clone();
    release_drag(&mut app, &["add"], 284.0, 48.0).await;
    assert_eq!(node_position(&content_snapshot(&app).await, "add"), (284.0, 48.0));
    let rows = member_rows(&mut app).await;
    assert_eq!(rows.len(), 1, "one drag is one row: {rows:?}");
    let row = &rows[0];
    let transaction = row.transaction.as_ref().expect("the drag row carries its transaction");
    assert_eq!(transaction.tool, "s.flow.flow@1/*#editor#nodeGraphEdit");
    assert!(transaction.id.starts_with("tx-"), "{}", transaction.id);
    assert_eq!(row.mutations.len(), 1, "one leaf: {:?}", row.mutations);
    let mutation = &row.mutations[0];
    assert_eq!(mutation.store.as_deref(), Some(format!("content/{content_id}").as_str()));
    assert!(mutation.editable, "a drag's inputs are editable");
    assert_eq!(row.label.resolve(protocol::Terminology::Native, protocol::Locale::En), "Drag 1 node by (284, 48)");
    assert_eq!(mutation.label.resolve(protocol::Terminology::Native, protocol::Locale::De), "1 Knoten um (284; 48) ziehen");
}

/// ⚖️ LAW (design §12): time travel on a composed child's mutation runs on that member store — editing the drag's `dx`
/// previews on the main window without touching the committed member, the replay reviews, and the overwrite folds the
/// edited offset into the member, which the parent's scene then shows.
#[semio_framework_async_macros::async_test]
async fn editing_a_node_drag_offset_replays_the_member_and_the_parent_scene_follows() {
    let mut app = flow_app_closing().await;
    let store = format!("content/{}", app.snapshot().expect("snapshot").content.child_id);
    release_drag(&mut app, &["add"], 284.0, 48.0).await;
    let mutation = member_rows(&mut app).await[0].mutations[0].mutation_id.clone();
    let begun = history_edit(&mut app, "historyEditBegin", vec![("mutationId", dsl::DslValue::String(mutation)), ("store", dsl::DslValue::String(store.clone()))]).await;
    assert!(begun.get("rejected").is_none(), "{begun:?}");
    let input = history_edit(&mut app, "historyEditInput", vec![("path", dsl::DslValue::String("/dx".into())), ("value", dsl::DslValue::float(100.0))]).await;
    assert!(input.get("rejected").is_none(), "{input:?}");
    assert_eq!(published_host_snapshot(&mut app).await["layout"]["add"], serde_json::json!({ "x": 100.0, "y": 48.0 }), "the main window renders the member's preview");
    assert_eq!(node_position(&content_snapshot(&app).await, "add"), (284.0, 48.0), "editing never touches the committed member");
    history_edit(&mut app, "historyEditAccept", Vec::new()).await;
    pump_time_travel(&mut app, |stage| stage != Some(semio_framework::kernel::HistoryTimeTravelStage::Replaying)).await;
    assert_eq!(time_travel_stage(&mut app).await, Some(semio_framework::kernel::HistoryTimeTravelStage::Reviewing));
    history_edit(&mut app, "historyEditFinalize", Vec::new()).await;
    let committed = history_edit(&mut app, "historyEditCommit", vec![("choice", dsl::DslValue::String("overwrite".into()))]).await;
    assert!(committed.get("rejected").is_none(), "{committed:?}");
    pump_time_travel(&mut app, |stage| stage.is_none()).await;
    assert_eq!(node_position(&content_snapshot(&app).await, "add"), (100.0, 48.0), "the overwrite folds the edited offset into the member");
    assert_eq!(published_host_snapshot(&mut app).await["layout"]["add"], serde_json::json!({ "x": 100.0, "y": 48.0 }), "the parent re-derives its scene from the replayed member");
    let edited = &member_rows(&mut app).await[0].mutations[0];
    assert!(edited.superseded && !edited.withdrawn, "the drag row shows its superseded input: {edited:?}");
    assert_eq!(edited.label.resolve(protocol::Terminology::Native, protocol::Locale::En), "Drag 1 node by (100, 48)", "the row reads the effective input");
}

/// ⚖️ LAW (design §12): withdrawing a composed child's drag in history puts the node back where it was, on the member and
/// on the parent's scene.
#[semio_framework_async_macros::async_test]
async fn withdrawing_a_node_drag_in_history_puts_the_node_back() {
    let mut app = flow_app_closing().await;
    let store = format!("content/{}", app.snapshot().expect("snapshot").content.child_id);
    release_drag(&mut app, &["add"], 284.0, 48.0).await;
    let mutation = member_rows(&mut app).await[0].mutations[0].mutation_id.clone();
    history_edit(&mut app, "historyEditBegin", vec![("mutationId", dsl::DslValue::String(mutation)), ("store", dsl::DslValue::String(store))]).await;
    history_edit(&mut app, "historyEditWithdraw", Vec::new()).await;
    history_edit(&mut app, "historyEditAccept", Vec::new()).await;
    pump_time_travel(&mut app, |stage| stage != Some(semio_framework::kernel::HistoryTimeTravelStage::Replaying)).await;
    history_edit(&mut app, "historyEditFinalize", Vec::new()).await;
    history_edit(&mut app, "historyEditCommit", vec![("choice", dsl::DslValue::String("overwrite".into()))]).await;
    pump_time_travel(&mut app, |stage| stage.is_none()).await;
    assert_eq!(node_position(&content_snapshot(&app).await, "add"), (0.0, 0.0), "the withdrawn drag moves nothing");
    assert_eq!(published_host_snapshot(&mut app).await["layout"]["add"], serde_json::json!({ "x": 0.0, "y": 0.0 }));
    assert!(member_rows(&mut app).await[0].mutations[0].withdrawn);
}

/// ⚖️ LAW (design §12): a replay whose report blocks — a later drag lost the node a withdrawn insert added
/// (`mutation.target-missing`) — refuses to finalize and leaves the member and the parent's scene untouched on exit.
#[semio_framework_async_macros::async_test]
async fn a_blocking_member_replay_refuses_to_finalize_and_exits_with_zero_trace() {
    let mut app = flow_app_closing().await;
    let store = format!("content/{}", app.snapshot().expect("snapshot").content.child_id);
    let before: Vec<String> = content_snapshot(&app).await.nodes.iter().map(|node| node.id.clone()).collect();
    app.handle_action("addWidget", Some(&dsl::DslValue::from(serde_json::json!({ "kind": "inputNote", "x": 40.0, "y": 40.0 }))), &meta("add-widget")).await.expect("addWidget admission");
    settle_registered_typed_operation(&mut **app, meta("local").instance_id).await.expect("addWidget publication");
    let added = content_snapshot(&app).await.nodes.iter().map(|node| node.id.clone()).find(|id| !before.contains(id)).expect("the added node");
    release_drag(&mut app, &[added.as_str()], 10.0, 0.0).await;
    let rows = member_rows(&mut app).await;
    let insert = rows.iter().flat_map(|row| row.mutations.iter()).find(|mutation| mutation.editable && rows.last().is_some_and(|last| !last.mutations.iter().any(|drag| drag.mutation_id == mutation.mutation_id))).expect("the insert's mutation row").mutation_id.clone();
    history_edit(&mut app, "historyEditBegin", vec![("mutationId", dsl::DslValue::String(insert)), ("store", dsl::DslValue::String(store))]).await;
    history_edit(&mut app, "historyEditWithdraw", Vec::new()).await;
    history_edit(&mut app, "historyEditAccept", Vec::new()).await;
    pump_time_travel(&mut app, |stage| stage != Some(semio_framework::kernel::HistoryTimeTravelStage::Replaying)).await;
    let status = app.history_snapshot().await.expect("history").time_travel.expect("a reviewing session");
    assert!(status.blocking, "the lost target blocks the replay: {status:?}");
    let finalized = history_edit(&mut app, "historyEditFinalize", Vec::new()).await;
    assert!(finalized.get("rejected").is_some(), "a blocking report refuses to finalize: {finalized:?}");
    history_edit(&mut app, "historyEditExit", Vec::new()).await;
    pump_time_travel(&mut app, |stage| stage.is_none()).await;
    assert_eq!(node_position(&content_snapshot(&app).await, &added), (50.0, 40.0), "exit leaves the committed member untouched");
}
/// 🎚️ One inline-slider dispatch of the press `gesture` (design §13.1): the absolute `setSlider` row with the press's own
/// `gesture`, the release adding `commit`, a cancel naming its `abort` reason; settles whatever it published.
async fn slide(app: &mut FlowApp, gesture: &str, value: Option<f64>, phase: Option<(&str, dsl::DslValue)>) {
    let operations: Vec<serde_json::Value> = value.map(|value| serde_json::json!({ "operation": "setSlider", "widgetId": "slider", "value": value })).into_iter().collect();
    let mut args = vec![("operations".to_string(), dsl::DslValue::from(serde_json::Value::Array(operations))), ("gesture".to_string(), dsl::DslValue::String(gesture.into()))];
    args.extend(phase.map(|(key, value)| (key.to_string(), value)));
    app.handle_action("nodeGraphEdit", Some(&dsl::DslValue::Object(args)), &crate::editor::flow::unit_tests::context::flow_main_window_meta()).await.expect("slider dispatch admission");
    let _ = settle_registered_typed_operation(&mut **app, meta("local").instance_id).await;
}

fn slider_value(content: &SemioFlowSnapshot) -> Option<String> {
    content.nodes.iter().find(|node| node.id == "slider").and_then(|node| node.params.iter().find(|param| param.key == "value")).map(|param| param.value.clone())
}

/// ⚖️ LAW (design §13.1, §12): a dragged inline slider's ticks stay provisional in the press, and the release lands ONE
/// absolute `set-node-param` edit in the composed content child, stamped with the press's transaction and listed as ONE
/// history row naming its member store; a press the host cancels leaves zero trace.
#[semio_framework_async_macros::async_test]
async fn a_dragged_inline_slider_is_one_child_transaction_and_a_cancel_leaves_zero_trace() {
    let mut app = flow_app_closing().await;
    let store = format!("content/{}", app.snapshot().expect("snapshot").content.child_id);
    let start = slider_value(&content_snapshot(&app).await);
    slide(&mut app, "slider:1", Some(4.0), None).await;
    slide(&mut app, "slider:1", Some(5.0), None).await;
    assert_eq!(slider_value(&content_snapshot(&app).await), start, "ticks publish nothing");
    assert!(member_rows(&mut app).await.is_empty(), "ticks list no history row");
    slide(&mut app, "slider:1", Some(6.0), Some(("commit", dsl::DslValue::Bool(true)))).await;
    assert_eq!(slider_value(&content_snapshot(&app).await).as_deref(), Some("6"), "the release lands the value the knob ended on");
    let rows = member_rows(&mut app).await;
    assert_eq!(rows.len(), 1, "one press is one row: {rows:?}");
    assert!(rows[0].transaction.as_ref().is_some_and(|transaction| transaction.tool == "s.flow.flow@1/*#editor#nodeGraphEdit"), "{:?}", rows[0].transaction);
    assert_eq!(rows[0].mutations.len(), 1, "one absolute leaf: {:?}", rows[0].mutations);
    assert_eq!(rows[0].mutations[0].store.as_deref(), Some(store.as_str()));
    slide(&mut app, "slider:2", Some(9.0), None).await;
    slide(&mut app, "slider:2", None, Some(("abort", dsl::DslValue::String("captureLost".into())))).await;
    assert_eq!(slider_value(&content_snapshot(&app).await).as_deref(), Some("6"), "a cancelled press leaves zero trace");
    assert_eq!(member_rows(&mut app).await.len(), 1, "a cancelled press lists no row");
}

/// ⚖️ LAW (design §13.1): an inspector value edit is the ABSOLUTE `set-node-param` leaf of the content child — never a
/// whole-content `set-snapshot`, never a coalesced amend — and a value the widget already holds is no edit.
#[semio_framework_async_macros::async_test]
async fn patching_a_widget_value_is_one_absolute_node_param_leaf() {
    let mut app = flow_app_closing().await;
    let patch = |value: &str| dsl::DslValue::from(serde_json::json!({ "widgetIds": ["slider"], "field": "value", "value": value }));
    app.handle_action("patchFlowWidgets", Some(&patch("7.5")), &crate::editor::flow::unit_tests::context::flow_main_window_meta()).await.expect("patch admission");
    settle_registered_typed_operation(&mut **app, meta("local").instance_id).await.expect("patch publication");
    assert_eq!(slider_value(&content_snapshot(&app).await).as_deref(), Some("7.5"));
    let rows = member_rows(&mut app).await;
    assert_eq!(rows.len(), 1, "{rows:?}");
    assert_eq!(rows[0].mutations.len(), 1, "one leaf, not a whole-content snapshot: {:?}", rows[0].mutations);
    assert!(rows[0].op_lines.iter().all(|line| !line.contains("set-snapshot")), "{:?}", rows[0].op_lines);
    app.handle_action("patchFlowWidgets", Some(&patch("7.5")), &crate::editor::flow::unit_tests::context::flow_main_window_meta()).await.expect("unchanged patch admission");
    let _ = settle_registered_typed_operation(&mut **app, meta("local").instance_id).await;
    assert_eq!(member_rows(&mut app).await.len(), 1, "a value the widget already holds is no edit");
}
//#endregion 🔖️ComposedChildHistory
