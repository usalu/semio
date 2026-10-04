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
    let connect = semio_framework_value::DslValue::from(serde_json::json!({
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
    let disconnect = semio_framework_value::DslValue::from(serde_json::json!({ "operations": [{ "operation": "disconnect", "synapseId": id }] }));
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

/// 🎯️ A `delete` row names what it deletes by id (design §13.3) — never the ambient selection — and lands as the intent
/// leaves of the content child: the widget's wires first, then the widget, never a whole-content `set-snapshot`.
#[semio_framework_async_macros::async_test]
async fn a_delete_row_removes_the_named_widget_and_its_wires_as_intent_leaves() {
    let mut app = flow_app_with_registry().await;
    let content_id = app.snapshot().expect("snapshot before the delete").content.child_id.clone();
    let wired: Vec<String> = content_snapshot(&app).await.edges.iter().filter(|edge| edge.from.node == "slider" || edge.to.node == "slider").map(|edge| edge.id.clone()).collect();
    select_graph(&mut app, &["add"], &[]).await;
    dispatch(&mut app, FlowCommand::NodeGraphEdit(NodeGraphEdit { operations: vec![FlowNodeGraphEditOp::Delete { node_ids: vec!["slider".into()], synapse_ids: Vec::new() }] })).await;
    let receipt = settle_registered_typed_operation(&mut *app, meta("local").instance_id).await.expect("delete child publication");
    assert_eq!(receipt.lanes, [TypedOperationResultLane::Child, TypedOperationResultLane::Ui, TypedOperationResultLane::Terminal]);
    assert_eq!(app.snapshot().expect("snapshot").content.child_id, content_id, "a delete publishes through the existing content child");
    let content = content_snapshot(&app).await;
    assert!(!content.nodes.iter().any(|node| node.id == "slider"), "the named widget is gone");
    assert!(content.nodes.iter().any(|node| node.id == "add"), "the selected but unnamed widget stays");
    let rows = member_rows(&mut app).await;
    let lines: Vec<&String> = rows.iter().flat_map(|row| row.op_lines.iter()).collect();
    assert_eq!(lines.iter().filter(|line| line.starts_with("remove-edge")).count(), wired.len(), "{lines:?}");
    assert_eq!(lines.iter().filter(|line| line.starts_with("remove-node")).count(), 1, "{lines:?}");
    assert!(lines.iter().all(|line| !line.starts_with("set-snapshot")), "{lines:?}");
    let _ = render(&mut app, crate::editor::flow::FLOW_PLAY_BODY_MAIN).await;
}

/// 🔌️ A `connect` row lands as ONE `insert-edge` leaf of the content child, never a whole-content `set-snapshot`.
#[semio_framework_async_macros::async_test]
async fn a_connect_row_is_one_insert_edge_leaf() {
    let mut app = flow_app_closing().await;
    let connect = semio_framework_value::DslValue::from(serde_json::json!({ "operations": [{ "operation": "connect", "sourceNodeId": "slider", "sourcePortId": "number", "targetNodeId": "add", "targetPortId": "b" }] }));
    app.handle_action("nodeGraphEdit", Some(&connect), &meta("renderer-wire-connect")).await.expect("connect admission");
    settle_registered_typed_operation(&mut *app, meta("local").instance_id).await.expect("connect publication");
    let rows = member_rows(&mut app).await;
    assert_eq!(rows.len(), 1, "{rows:?}");
    assert_eq!(rows[0].mutations.len(), 1, "{:?}", rows[0].op_lines);
    assert!(rows[0].op_lines.iter().all(|line| line.starts_with("insert-edge")), "{:?}", rows[0].op_lines);
    assert!(rows[0].mutations[0].editable, "an inserted wire is editable in history");
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
    let args = semio_framework_value::DslValue::from(serde_json::json!({
        "operations": [
            { "operation": "delete", "nodeIds": ["add"], "synapseIds": ["s2"] },
            { "operation": "insertPort", "nodeId": "add", "side": "output", "index": 1 },
            { "operation": "connect", "sourceNodeId": "slider", "sourcePortId": "number", "targetNodeId": "add", "targetPortId": "a" },
            { "operation": "disconnect", "synapseId": "s1" },
            { "operation": "move", "gestureId": "node-drag:3", "nodeIds": ["add", "slider"], "dx": 284.0, "dy": 48.0 }
        ]
    }));
    assert_eq!(
        operations_from_action(&args).expect("the renderer's current operation vocabulary"),
        vec![
            FlowNodeGraphEditOp::Delete { node_ids: vec!["add".into()], synapse_ids: vec!["s2".into()] },
            FlowNodeGraphEditOp::InsertPort { node_id: "add".into(), side: "output".into(), index: 1 },
            FlowNodeGraphEditOp::Connect { source_node_id: "slider".into(), source_port_id: "number".into(), target_node_id: "add".into(), target_port_id: "a".into() },
            FlowNodeGraphEditOp::Disconnect { synapse_id: "s1".into() },
            FlowNodeGraphEditOp::Move { gesture_id: "node-drag:3".into(), node_ids: vec!["add".into(), "slider".into()], dx: 284.0, dy: 48.0 },
        ]
    );
    let malformed = semio_framework_value::DslValue::from(serde_json::json!({
        "operations": [
            { "operation": "move", "gestureId": "node-drag:3", "nodeIds": ["add"], "dx": 284.0, "dy": 48.0 },
            { "operation": "move", "nodeId": "add", "x": 568.0, "y": 96.0 }
        ]
    }));
    assert!(operations_from_action(&malformed).is_err(), "one malformed row must refuse the complete operation array");
    for refused in [
        serde_json::json!({ "operation": "setHostSnapshot", "hostSnapshotJson": "{}" }),
        serde_json::json!({ "operation": "deleteSelection" }),
        serde_json::json!({ "operation": "delete", "nodeIds": [], "synapseIds": [] }),
        serde_json::json!({ "operation": "delete", "nodeIds": ["add", "add"], "synapseIds": [] }),
        serde_json::json!({ "operation": "insertPort", "nodeId": "add", "side": "left", "index": 1 }),
        serde_json::json!({ "operation": "insertPort", "nodeId": "add", "side": "input", "index": 1.5 }),
    ] {
        assert!(operations_from_action(&semio_framework_value::DslValue::from(serde_json::json!({ "operations": [refused.clone()] }))).is_err(), "{refused} is no row");
    }
}

#[semio_framework_async_macros::async_test]
async fn operation_parser_refuses_beyond_the_retained_route_row_and_wire_authorities() {
    let too_many = semio_framework_value::DslValue::from(serde_json::json!({
        "operations": (0..=crate::editor::flow::FLOW_STORE_MAX_MUTATION_ITEMS).map(|_| serde_json::json!({ "operation": "deleteSelection" })).collect::<Vec<_>>()
    }));
    assert!(operations_from_action(&too_many).is_err(), "the parser must refuse before walking row 257");

    let oversized = semio_framework_value::DslValue::from(serde_json::json!({
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
    let args = semio_framework_value::DslValue::from(serde_json::json!({
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
    let args = semio_framework_value::DslValue::from(serde_json::json!({
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
    release_drag_as(app, &meta("renderer-node-drag"), nodes, dx, dy).await;
}

/// 🎭️ [`release_drag`] dispatched by `metadata`'s actor on `metadata`'s instance.
async fn release_drag_as(app: &mut FlowApp, metadata: &semio_framework_plugin::ActionMeta, nodes: &[&str], dx: f64, dy: f64) {
    let args = semio_framework_value::DslValue::from(serde_json::json!({ "operations": [{ "operation": "move", "gestureId": "node-drag:9", "nodeIds": nodes, "dx": dx, "dy": dy }] }));
    app.handle_action("nodeGraphEdit", Some(&args), metadata).await.expect("node drag admission");
    settle_registered_typed_operation(app, metadata.instance_id).await.expect("node drag publication");
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

/// ⏪️ Runs one history-edit verb and answers its output (a refusal is a `{rejected}` output, a fault fails the law).
async fn history_edit(app: &mut FlowApp, verb: &str, args: Vec<(&str, semio_framework_value::DslValue)>) -> semio_framework_value::DslValue {
    history_edit_as(app, &meta("history-edit"), verb, args).await
}

/// 🎭️ [`history_edit`] dispatched by `metadata`'s actor on `metadata`'s instance.
async fn history_edit_as(app: &mut FlowApp, metadata: &semio_framework_plugin::ActionMeta, verb: &str, args: Vec<(&str, semio_framework_value::DslValue)>) -> semio_framework_value::DslValue {
    let args = semio_framework_value::DslValue::Object(args.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
    app.handle_action(verb, Some(&args), metadata).await.unwrap_or_else(|fault| panic!("{verb}: {fault:?}")).output
}

/// ✏️ Edits the drag `mutation` of member `store` to `dx = 100` through time travel, accepts it and waits for the review.
async fn edit_drag_offset(app: &mut FlowApp, metadata: &semio_framework_plugin::ActionMeta, store: &str, mutation: String) {
    let begun = history_edit_as(app, metadata, "historyEditBegin", vec![("mutationId", semio_framework_value::DslValue::String(mutation)), ("store", semio_framework_value::DslValue::String(store.into()))]).await;
    assert!(begun.get("rejected").is_none(), "{begun:?}");
    let input = history_edit_as(app, metadata, "historyEditInput", vec![("path", semio_framework_value::DslValue::String("/dx".into())), ("value", semio_framework_value::DslValue::float(100.0))]).await;
    assert!(input.get("rejected").is_none(), "{input:?}");
    history_edit_as(app, metadata, "historyEditAccept", Vec::new()).await;
    pump_time_travel(app, |stage| stage != Some(semio_framework::kernel::HistoryTimeTravelStage::Replaying)).await;
    assert_eq!(time_travel_stage(app).await, Some(semio_framework::kernel::HistoryTimeTravelStage::Reviewing));
}

/// 🌿️ Finalizes the reviewed session as the new alternative `name` and waits until it closed.
async fn finalize_as_alternative(app: &mut FlowApp, metadata: &semio_framework_plugin::ActionMeta, name: &str) {
    history_edit_as(app, metadata, "historyEditFinalize", Vec::new()).await;
    let committed = history_edit_as(app, metadata, "historyEditCommit", vec![("name", semio_framework_value::DslValue::String(name.into()))]).await;
    assert!(committed.get("rejected").is_none(), "{committed:?}");
    pump_time_travel(app, |stage| stage.is_none()).await;
}

async fn member_alternative(app: &FlowApp) -> Option<String> {
    let parent = app.snapshot().expect("Flow parent snapshot");
    app.child_store("content", &parent.content.child_id).await.expect("Flow content child").current_alternative_id().await
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
    assert_eq!(row.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), "Drag 1 node by (284, 48)");
    assert_eq!(mutation.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), "1 Knoten um (284; 48) ziehen");
}

/// ⚖️ LAW (design §19.1, audit F5): a release that drew a wire and dragged a node is ONE row labelled by the drag, though
/// the wire's `insert-edge` lands before the relative `drag-nodes`.
#[semio_framework_async_macros::async_test]
async fn a_release_that_wires_and_drags_is_labelled_by_the_drag() {
    let mut app = flow_app_closing().await;
    let args = semio_framework_value::DslValue::from(serde_json::json!({ "operations": [
        { "operation": "connect", "sourceNodeId": "slider", "sourcePortId": "number", "targetNodeId": "add", "targetPortId": "b" },
        { "operation": "move", "gestureId": "node-drag:4", "nodeIds": ["add"], "dx": 10.0, "dy": 0.0 }
    ] }));
    app.handle_action("nodeGraphEdit", Some(&args), &meta("renderer-node-drag")).await.expect("wire and drag admission");
    settle_registered_typed_operation(&mut *app, meta("local").instance_id).await.expect("wire and drag publication");
    let rows = member_rows(&mut app).await;
    assert_eq!(rows.len(), 1, "one release is one row: {rows:?}");
    assert!(rows[0].op_lines.first().is_some_and(|line| line.starts_with("insert-edge")), "{:?}", rows[0].op_lines);
    assert_eq!(rows[0].label.resolve(protocol::Terminology::Native, protocol::Locale::En), "Drag 1 node by (10, 0)");
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
    let begun = history_edit(&mut app, "historyEditBegin", vec![("mutationId", semio_framework_value::DslValue::String(mutation)), ("store", semio_framework_value::DslValue::String(store.clone()))]).await;
    assert!(begun.get("rejected").is_none(), "{begun:?}");
    let input = history_edit(&mut app, "historyEditInput", vec![("path", semio_framework_value::DslValue::String("/dx".into())), ("value", semio_framework_value::DslValue::float(100.0))]).await;
    assert!(input.get("rejected").is_none(), "{input:?}");
    assert_eq!(published_host_snapshot(&mut app).await["layout"]["add"], serde_json::json!({ "x": 100.0, "y": 48.0 }), "the main window renders the member's preview");
    assert_eq!(node_position(&content_snapshot(&app).await, "add"), (284.0, 48.0), "editing never touches the committed member");
    history_edit(&mut app, "historyEditAccept", Vec::new()).await;
    pump_time_travel(&mut app, |stage| stage != Some(semio_framework::kernel::HistoryTimeTravelStage::Replaying)).await;
    assert_eq!(time_travel_stage(&mut app).await, Some(semio_framework::kernel::HistoryTimeTravelStage::Reviewing));
    history_edit(&mut app, "historyEditFinalize", Vec::new()).await;
    let committed = history_edit(&mut app, "historyEditCommit", vec![("choice", semio_framework_value::DslValue::String("overwrite".into()))]).await;
    assert!(committed.get("rejected").is_none(), "{committed:?}");
    pump_time_travel(&mut app, |stage| stage.is_none()).await;
    assert_eq!(node_position(&content_snapshot(&app).await, "add"), (100.0, 48.0), "the overwrite folds the edited offset into the member");
    assert_eq!(store, format!("content/{}", app.snapshot().expect("snapshot").content.child_id), "the parent never re-mints the live child");
    assert_eq!(published_host_snapshot(&mut app).await["layout"]["add"], serde_json::json!({ "x": 100.0, "y": 48.0 }), "the parent re-derives its scene from the replayed member");
    let edited = &member_rows(&mut app).await[0].mutations[0];
    assert!(edited.superseded && !edited.withdrawn, "the drag row shows its superseded input: {edited:?}");
    assert_eq!(edited.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), "Drag 1 node by (100, 48)", "the row reads the effective input");
}

/// ⚖️ LAW (design §12, §9.5): finalizing a member edit as a NEW ALTERNATIVE branches the member store (the commit of its
/// pending edits, a `Branch` at its head, the supersession scoped to the branch) and views the branch; the parent keeps
/// the live child — same id, never re-minted or retired — and re-derives its scene from the branch's replayed head.
#[semio_framework_async_macros::async_test]
async fn finalizing_a_member_edit_as_a_new_alternative_branches_the_member_and_keeps_the_live_child() {
    let mut app = flow_app_closing().await;
    let store = format!("content/{}", app.snapshot().expect("snapshot").content.child_id);
    release_drag(&mut app, &["add"], 284.0, 48.0).await;
    let line = member_alternative(&app).await;
    let mutation = member_rows(&mut app).await[0].mutations[0].mutation_id.clone();
    edit_drag_offset(&mut app, &meta("history-edit"), &store, mutation).await;
    finalize_as_alternative(&mut app, &meta("history-edit"), "Closer").await;
    assert_eq!(store, format!("content/{}", app.snapshot().expect("snapshot").content.child_id), "the parent never re-mints the live child");
    let branch = member_alternative(&app).await;
    assert!(branch.is_some() && branch != line, "the member views its new alternative: {line:?} -> {branch:?}");
    assert_eq!(node_position(&content_snapshot(&app).await, "add"), (100.0, 48.0), "the branch folds the edited offset");
    assert_eq!(published_host_snapshot(&mut app).await["layout"]["add"], serde_json::json!({ "x": 100.0, "y": 48.0 }), "the parent re-derives its scene from the branch");
    let edited = &member_rows(&mut app).await[0].mutations[0];
    assert!(edited.superseded && !edited.withdrawn, "on the branch the drag row shows its superseded input: {edited:?}");
}

/// ⚖️ LAW (design §12, §2): a member finalize reaches the other replica on the member's lane of the parent's backbone —
/// every transition it authored, so a new alternative's commit and `Branch` arrive with its scoped `Supersede` — and both
/// replicas converge on the edited member and the same member alternative.
#[semio_framework_async_macros::async_test]
async fn a_member_finalize_reaches_the_other_replica_on_the_member_lane() {
    use store::MemoryBackbone;
    let mut instance_a = crate::editor::flow::unit_tests::context::flow_app_with_registry().await;
    let mut instance_b = crate::editor::flow::unit_tests::context::flow_app_with_registry().await;
    let metadata_a = semio_framework_plugin::ActionMeta { instance_id: 73, ..meta("actor-a") };
    let metadata_b = semio_framework_plugin::ActionMeta { instance_id: 74, ..meta("actor-b") };
    instance_a.bind_instance_id(metadata_a.instance_id).await;
    instance_b.bind_instance_id(metadata_b.instance_id).await;
    let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://flow-member-history", "mem://flow-member-history").await;
    instance_a.attach_backbone(store::Backbones::Memory(backbone_a)).await.expect("attach a");
    instance_b.attach_backbone(store::Backbones::Memory(backbone_b)).await.expect("attach b");
    let store = format!("content/{}", instance_a.snapshot().expect("snapshot").content.child_id);
    release_drag_as(&mut instance_a, &metadata_a, &["add"], 284.0, 48.0).await;
    instance_b.tick_backbone().await.expect("b folds a's drag");
    assert_eq!(node_position(&content_snapshot(&instance_b).await, "add"), (284.0, 48.0), "b holds a's drag");
    let mutation = member_rows(&mut instance_a).await[0].mutations[0].mutation_id.clone();
    edit_drag_offset(&mut instance_a, &metadata_a, &store, mutation).await;
    finalize_as_alternative(&mut instance_a, &metadata_a, "Closer").await;
    instance_b.tick_backbone().await.expect("b folds a's member finalize");
    assert_eq!(node_position(&content_snapshot(&instance_b).await, "add"), (100.0, 48.0), "b folds the edited offset");
    assert_eq!(member_alternative(&instance_b).await, member_alternative(&instance_a).await, "both replicas view the same member alternative");
    assert_eq!(content_snapshot(&instance_a).await.nodes.len(), content_snapshot(&instance_b).await.nodes.len());
    instance_a.detach_backbone().await.expect("a releases its backbone");
    instance_b.detach_backbone().await.expect("b releases its backbone");
}

/// ⚖️ LAW (design §12): withdrawing a composed child's drag in history puts the node back where it was, on the member and
/// on the parent's scene.
#[semio_framework_async_macros::async_test]
async fn withdrawing_a_node_drag_in_history_puts_the_node_back() {
    let mut app = flow_app_closing().await;
    let store = format!("content/{}", app.snapshot().expect("snapshot").content.child_id);
    release_drag(&mut app, &["add"], 284.0, 48.0).await;
    let mutation = member_rows(&mut app).await[0].mutations[0].mutation_id.clone();
    history_edit(&mut app, "historyEditBegin", vec![("mutationId", semio_framework_value::DslValue::String(mutation)), ("store", semio_framework_value::DslValue::String(store))]).await;
    history_edit(&mut app, "historyEditWithdraw", Vec::new()).await;
    history_edit(&mut app, "historyEditAccept", Vec::new()).await;
    pump_time_travel(&mut app, |stage| stage != Some(semio_framework::kernel::HistoryTimeTravelStage::Replaying)).await;
    history_edit(&mut app, "historyEditFinalize", Vec::new()).await;
    history_edit(&mut app, "historyEditCommit", vec![("choice", semio_framework_value::DslValue::String("overwrite".into()))]).await;
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
    app.handle_action("addWidget", Some(&semio_framework_value::DslValue::from(serde_json::json!({ "kind": "inputNote", "x": 40.0, "y": 40.0 }))), &meta("add-widget")).await.expect("addWidget admission");
    settle_registered_typed_operation(&mut *app, meta("local").instance_id).await.expect("addWidget publication");
    let added = content_snapshot(&app).await.nodes.iter().map(|node| node.id.clone()).find(|id| !before.contains(id)).expect("the added node");
    release_drag(&mut app, &[added.as_str()], 10.0, 0.0).await;
    let rows = member_rows(&mut app).await;
    let insert = rows.iter().flat_map(|row| row.mutations.iter()).find(|mutation| mutation.editable && rows.last().is_some_and(|last| !last.mutations.iter().any(|drag| drag.mutation_id == mutation.mutation_id))).expect("the insert's mutation row").mutation_id.clone();
    history_edit(&mut app, "historyEditBegin", vec![("mutationId", semio_framework_value::DslValue::String(insert)), ("store", semio_framework_value::DslValue::String(store))]).await;
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
async fn slide(app: &mut FlowApp, gesture: &str, value: Option<f64>, phase: Option<(&str, semio_framework_value::DslValue)>) {
    let operations: Vec<serde_json::Value> = value.map(|value| serde_json::json!({ "operation": "setSlider", "widgetId": "slider", "value": value })).into_iter().collect();
    let mut args = vec![("operations".to_string(), semio_framework_value::DslValue::from(serde_json::Value::Array(operations))), ("gesture".to_string(), semio_framework_value::DslValue::String(gesture.into()))];
    args.extend(phase.map(|(key, value)| (key.to_string(), value)));
    app.handle_action("nodeGraphEdit", Some(&semio_framework_value::DslValue::Object(args)), &crate::editor::flow::unit_tests::context::flow_main_window_meta()).await.expect("slider dispatch admission");
    let _ = settle_registered_typed_operation(app, meta("local").instance_id).await;
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
    slide(&mut app, "slider:1", Some(6.0), Some(("commit", semio_framework_value::DslValue::Bool(true)))).await;
    assert_eq!(slider_value(&content_snapshot(&app).await).as_deref(), Some("6"), "the release lands the value the knob ended on");
    let rows = member_rows(&mut app).await;
    assert_eq!(rows.len(), 1, "one press is one row: {rows:?}");
    assert!(rows[0].transaction.as_ref().is_some_and(|transaction| transaction.tool == "s.flow.flow@1/*#editor#nodeGraphEdit"), "{:?}", rows[0].transaction);
    assert_eq!(rows[0].mutations.len(), 1, "one absolute leaf: {:?}", rows[0].mutations);
    assert_eq!(rows[0].mutations[0].store.as_deref(), Some(store.as_str()));
    slide(&mut app, "slider:2", Some(9.0), None).await;
    slide(&mut app, "slider:2", None, Some(("abort", semio_framework_value::DslValue::String("captureLost".into())))).await;
    assert_eq!(slider_value(&content_snapshot(&app).await).as_deref(), Some("6"), "a cancelled press leaves zero trace");
    assert_eq!(member_rows(&mut app).await.len(), 1, "a cancelled press lists no row");
}

/// ⚖️ LAW (design §13.1): an inspector value edit is the ABSOLUTE `set-node-param` leaf of the content child — never a
/// whole-content `set-snapshot`, never a coalesced amend — and a value the widget already holds is no edit.
#[semio_framework_async_macros::async_test]
async fn patching_a_widget_value_is_one_absolute_node_param_leaf() {
    let mut app = flow_app_closing().await;
    let patch = |value: &str| semio_framework_value::DslValue::from(serde_json::json!({ "widgetIds": ["slider"], "field": "value", "value": value }));
    app.handle_action("patchFlowWidgets", Some(&patch("7.5")), &crate::editor::flow::unit_tests::context::flow_main_window_meta()).await.expect("patch admission");
    settle_registered_typed_operation(&mut *app, meta("local").instance_id).await.expect("patch publication");
    assert_eq!(slider_value(&content_snapshot(&app).await).as_deref(), Some("7.5"));
    let rows = member_rows(&mut app).await;
    assert_eq!(rows.len(), 1, "{rows:?}");
    assert_eq!(rows[0].mutations.len(), 1, "one leaf, not a whole-content snapshot: {:?}", rows[0].mutations);
    assert!(rows[0].op_lines.iter().all(|line| !line.contains("set-snapshot")), "{:?}", rows[0].op_lines);
    app.handle_action("patchFlowWidgets", Some(&patch("7.5")), &crate::editor::flow::unit_tests::context::flow_main_window_meta()).await.expect("unchanged patch admission");
    let _ = settle_registered_typed_operation(&mut *app, meta("local").instance_id).await;
    assert_eq!(member_rows(&mut app).await.len(), 1, "a value the widget already holds is no edit");
}
//#endregion 🔖️ComposedChildHistory

//#region 🔖️IntentRows
const NODE_GRAPH_EDIT_ROWS_JSON: &str = include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json");

/// ⚖️ LAW (design §13.3): the flow guest decodes every row of the shared node-graph edit vocabulary the fixture accepts
/// and refuses every row it refuses — among them the whole-fixture `setHostSnapshot` and the ambient `deleteSelection`.
#[semio_framework_async_macros::async_test]
async fn the_guest_reads_exactly_the_shared_node_graph_edit_rows() {
    let fixture: serde_json::Value = serde_json::from_str(NODE_GRAPH_EDIT_ROWS_JSON).expect("node-graph edit rows fixture");
    for case in fixture["accepted"].as_array().expect("accepted rows") {
        let args = semio_framework_value::DslValue::from(serde_json::json!({ "operations": [case["row"].clone()] }));
        assert_eq!(operations_from_action(&args).map(|rows| rows.len()).ok(), Some(1), "{} decodes", case["id"]);
    }
    for case in fixture["refused"].as_array().expect("refused rows") {
        let args = semio_framework_value::DslValue::from(serde_json::json!({ "operations": [case["row"].clone()] }));
        assert!(operations_from_action(&args).is_err(), "{} is refused", case["id"]);
    }
    assert_eq!(flow::dag::DAG_GRAPH_EDIT_CAPACITY, semio_framework_tool_machine::NODE_GRAPH_EDIT_MAX_ROWS, "a host journal never holds more rows than one dispatch admits");
}

fn flow_node(id: &str, kind: &str, params: &[(&str, &str)], x: f64) -> semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::FlowNode {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::{FlowNode, FlowParam};
    let params = params.iter().map(|(key, value)| FlowParam { key: (*key).into(), value: (*value).into() }).collect();
    FlowNode { id: id.into(), kind: kind.into(), label: id.into(), params, position: semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint2 { x, y: 0.0 } }
}

fn flow_edge(id: &str, from: &str, to: &str) -> semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::FlowEdge {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::{FlowEdge, PortRef};
    FlowEdge { id: id.into(), from: PortRef { node: from.into(), port: "out".into() }, to: PortRef { node: to.into(), port: "in".into() }, kind: "data".into() }
}

/// 🧮️ `leaves` folded over `base` exactly as the content child's store folds them; every leaf lands without a message.
fn fold_leaves(base: &SemioFlowSnapshot, leaves: &[semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::SemioFlowMutation]) -> SemioFlowSnapshot {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::SemioFlowMutation;
    leaves.iter().fold(base.clone(), |state, leaf| {
        let (diff, messages) = <SemioFlowMutation as protocol::Mutation<SemioFlowSnapshot>>::diff(leaf, &state).into_parts();
        assert!(messages.is_empty(), "{leaf:?}: {messages:?}");
        <<SemioFlowMutation as protocol::Mutation<SemioFlowSnapshot>>::Diff as protocol::MutationDiff<SemioFlowSnapshot>>::apply(&diff, &state).unwrap_or_else(|error| panic!("{leaf:?}: {error:?}"))
    })
}

/// 🔤️ `snapshot` with nodes, edges and params sorted by id/key — the identity an id-keyed edit preserves.
fn by_id(mut snapshot: SemioFlowSnapshot) -> SemioFlowSnapshot {
    snapshot.nodes.sort_by(|a, b| a.id.cmp(&b.id));
    snapshot.edges.sort_by(|a, b| a.id.cmp(&b.id));
    snapshot.nodes.iter_mut().for_each(|node| node.params.sort_by(|a, b| a.key.cmp(&b.key)));
    snapshot
}

/// ⚖️ LAW (design §12, §20.3): a host edit lands as the intent leaves that turn the content child into the edited scene —
/// folding them reproduces the scene by id, no leaf ever dangles, each edit is the leaf kinds its intent names, and a pure
/// reorder is no edit.
#[test]
fn host_edits_land_as_the_intent_leaves_that_reproduce_the_scene() {
    let base = SemioFlowSnapshot { nodes: vec![flow_node("a", "inputSlider", &[("value", "1"), ("min", "0")], 0.0), flow_node("b", "math.add", &[], 100.0), flow_node("c", "outputPreview", &[], 200.0)], edges: vec![flow_edge("e1", "a", "b"), flow_edge("e2", "b", "c")], ..SemioFlowSnapshot::default() };
    let kinds = |leaves: &[semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::SemioFlowMutation]| -> Vec<String> { leaves.iter().map(|leaf| protocol::OpText::print_op(leaf).split(' ').next().unwrap_or_default().to_string()).collect() };
    let mut deleted = base.clone();
    deleted.nodes.retain(|node| node.id != "b");
    deleted.edges.clear();
    let mut renamed = base.clone();
    renamed.nodes[1].id = "sum".into();
    renamed.edges = vec![flow_edge("e1", "a", "sum"), flow_edge("e2", "sum", "c")];
    let mut edited = base.clone();
    edited.nodes[0] = flow_node("a", "inputSlider", &[("value", "6"), ("max", "9")], 40.0);
    edited.nodes[2].kind = "outputExport".into();
    edited.nodes.push(flow_node("d", "inputNote", &[("text", "hi")], 300.0));
    edited.edges.push(flow_edge("e3", "d", "c"));
    let mut reordered = base.clone();
    reordered.nodes.reverse();
    reordered.edges.reverse();
    let cases: [(&str, &SemioFlowSnapshot, Vec<&str>); 4] = [
        ("delete a node with its wires", &deleted, vec!["remove-edge", "remove-edge", "remove-node"]),
        ("rename a wired node", &renamed, vec!["insert-node", "set-edge-endpoints", "set-edge-endpoints", "remove-node"]),
        ("edit fields, add a wired node", &edited, vec!["set-node-position", "remove-node-param", "set-node-param", "set-node-param", "set-node-kind", "insert-node", "insert-edge"]),
        ("reorder", &reordered, Vec::new()),
    ];
    for (what, next, expected) in cases {
        let leaves = crate::editor::flow::flow_content_leaves(&base, next);
        assert_eq!(kinds(&leaves), expected, "{what}");
        assert_eq!(by_id(fold_leaves(&base, &leaves)), by_id(next.clone()), "{what}");
    }
}
//#endregion 🔖️IntentRows

