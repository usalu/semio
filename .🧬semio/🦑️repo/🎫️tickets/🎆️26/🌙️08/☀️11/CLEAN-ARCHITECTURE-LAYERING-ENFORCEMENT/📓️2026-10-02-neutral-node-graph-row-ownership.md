# Neutral Node Graph Row Ownership

The actual second registered whole-Rust census refused the general ToolMachine unit importing an OS renderer fixture. This is a source-first ownership cut; the framework owns its closed generic row vocabulary. Renderer journal sanitization and flow add-widget descriptors remain in their specific renderer owner.

All original eight accepted and fourteen refused row values, four journal answers, five add-widget descriptors, and every existing consumer assertion are retained. Only explicit fixture/schema imports and documentation change. No enforcer exception, facade, forward, runtime library or legacy path is introduced.

The generic root schema will also declare the three optional opaque scrub fields already accepted by the unchanged Rust batch decoder (gesture, commit, abort); their typed decoding belongs to the scrub machine. This does not change that decoder.

Original generic case canonical SHA-256: 01322c8975adc09ced12d9c9eef1cc2d45d66f8b607c2c925d04507087217a31.

New registered source/schema route will be @semio-tech/framework-tool-machine-rs:test-node-graph-row-ownership; unchanged original full native ToolMachine test is required separately. Neither products-absent compilation nor full runtime is claimed.

## Original Inputs

### 🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs

SHA-256 090ece4862207cbba0ad0994f55041b221084f7d979d8a058f4edf62d8029c12; 5401 bytes.

```rust

use super::*;

#[semio_framework_async_macros::async_test]
async fn space_command_op_text_round_trips_every_variant() {
    use crate::engine::space::SpaceCommand;
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::NodeGraphEdit(NodeGraphEdit { operations_json: "[]".into() }));
}

//#region ✋️GestureLaws
/// 🕹️ The `nodeGraphEdit` operations array a host dispatches, as its JSON text.
fn rows(rows: serde_json::Value) -> NodeGraphEdit {
    NodeGraphEdit { operations_json: rows.to_string() }
}

/// ⚖️ LAW: a released node drag (the node-graph gesture record, as both hosts write it) is ONE relative `move-nodes` leaf
/// over the record's nodes the workflow holds; a ghost node is dropped from the record, and a drag that moves nothing
/// leaves zero trace.
#[semio_framework_async_macros::async_test]
async fn a_node_drag_record_is_one_relative_move() {
    use crate::demo_space_projection;
    use crate::engine::space::SpaceCommand;
    use crate::engine::space::unit_tests::context::{apply_mutations, studio_emit};
    let projection = demo_space_projection().await;
    let config = SpaceConfig::default();
    let ids: Vec<String> = projection.graph.nodes.iter().take(2).map(|node| node.id.clone()).collect();
    let mut record_ids = ids.clone();
    record_ids.push("ghost".into());
    let emit = studio_emit(&projection, &config, &SpaceCommand::NodeGraphEdit(rows(serde_json::json!([{ "operation": "move", "gestureId": "node-drag:1", "nodeIds": record_ids, "dx": 40.0, "dy": -12.5 }])))).await.expect("handle");
    assert!(matches!(emit.artifact_mutations.as_slice(), [WorkflowMutation::MoveNodes(leaf)] if leaf.node_ids == ids && leaf.dx == 40.0 && leaf.dy == -12.5), "{:?}", emit.artifact_mutations);
    let moved = apply_mutations(&projection, &emit.artifact_mutations).await;
    for id in &ids {
        let (before, after) = (projection.graph.nodes.iter().find(|node| &node.id == id).expect("base"), moved.graph.nodes.iter().find(|node| &node.id == id).expect("moved"));
        assert_eq!((after.x, after.y), (before.x + 40.0, before.y - 12.5));
    }
    for nothing in [serde_json::json!([{ "operation": "move", "gestureId": "node-drag:2", "nodeIds": [ids[0]], "dx": 0.0, "dy": 0.0 }]), serde_json::json!([{ "operation": "move", "gestureId": "node-drag:3", "nodeIds": ["ghost"], "dx": 5.0, "dy": 0.0 }])] {
        let emit = studio_emit(&projection, &config, &SpaceCommand::NodeGraphEdit(rows(nothing))).await.expect("handle");
        assert!(emit.artifact_mutations.is_empty() && emit.transaction.is_none(), "zero trace: {:?}", emit.artifact_mutations);
    }
}

/// ⚖️ LAW: a `delete` row removes exactly the edges and app instances it names (never an ambient selection), edges first.
#[semio_framework_async_macros::async_test]
async fn a_delete_row_removes_the_named_edges_then_nodes() {
    use crate::demo_space_projection;
    use crate::engine::space::SpaceCommand;
    use crate::engine::space::unit_tests::context::studio_emit;
    let projection = demo_space_projection().await;
    let config = SpaceConfig::default();
    let node = projection.graph.nodes.first().expect("node").id.clone();
    let edge = projection.graph.edges.first().map(|edge| edge.id.clone());
    let emit = studio_emit(&projection, &config, &SpaceCommand::NodeGraphEdit(rows(serde_json::json!([{ "operation": "delete", "nodeIds": [node, "ghost"], "synapseIds": edge.iter().collect::<Vec<_>>() }])))).await.expect("handle");
    let disconnects = emit.artifact_mutations.iter().filter(|leaf| matches!(leaf, WorkflowMutation::DisconnectEdge(_))).count();
    assert_eq!(disconnects, usize::from(edge.is_some()), "{:?}", emit.artifact_mutations);
    assert!(matches!(emit.artifact_mutations.last(), Some(WorkflowMutation::RemoveNode(leaf)) if leaf.node_id == node), "{:?}", emit.artifact_mutations);
    assert_eq!(emit.artifact_mutations.len(), disconnects + 1, "the ghost node is skipped");
}

/// ⚖️ LAW: the studio guest decodes the renderer's committed node-graph rows through the ONE shared decoder: every refused
/// row is refused, every accepted row a workflow carries (`move`, `connect`, `disconnect`, `delete`) decodes, and the
/// `setSlider`/`insertPort` rows it has no widget for are refused.
#[test]
fn the_renderer_row_fixture_decodes_exactly() {
    let fixture: serde_json::Value = serde_json::from_str(NODE_GRAPH_EDIT_ROWS).expect("the row fixture parses");
    let row = |value: &serde_json::Value| space_node_graph_row(&pack::parse_json(&value.to_string()).expect("row JSON"));
    for case in fixture["accepted"].as_array().expect("accepted rows") {
        let carried = !matches!(case["row"]["operation"].as_str(), Some("setSlider" | "insertPort"));
        assert_eq!(row(&case["row"]).is_ok(), carried, "accepted row {}", case["id"]);
    }
    for case in fixture["refused"].as_array().expect("refused rows") {
        assert!(row(&case["row"]).is_err(), "refused row {} decoded", case["id"]);
    }
}

/// 🧾️ The renderer's committed node-graph row vocabulary (schema `📺️renderer/🧑‍🎨engine/🧬️schema/🔣️node-graph-edit-rows`).
const NODE_GRAPH_EDIT_ROWS: &str = include_str!("../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json");
//#endregion ✋️GestureLaws

```

### ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs

SHA-256 0acaa7247e175b3d7b019aed86a15d8b7179b6aacc0313ea7153483f0a42bfde; 19450 bytes.

```rust
use super::DagNodeGraphEditOp;
use super::*;
use crate::editor::dag::commands::{connect_media_ports, disconnect, patch_dag_nodes};
use crate::editor::dag::unit_tests::context;
use crate::editor::dag::unit_tests::context::DagApp;
use crate::editor::dag::DagCommand;
use semio_framework::kernel::HistoryEntry;
use semio_framework_plugin::artifact_app_laws::{meta, settle_history_verb, settle_registered_typed_operation};
use semio_framework_plugin::PluginApp;
use serde_json::json;

/// 🧪️ `nodeGraphEdit` batches id-keyed sub-edits: a `connect` row adds an edge, and a `delete` row removes exactly the
/// nodes it names together with every wire they hold — never an ambient selection.
#[semio_framework_async_macros::async_test]
async fn node_graph_edit_connects_then_deletes_the_named_node() {
    let mut app = context::new_app().await;
    let (source_id, target_id) = {
        let projection = app.snapshot().expect("projection");
        let nodes = projection.nodes();
        (nodes[0].id.clone(), nodes[1].id.clone())
    };
    let edges_before = app.snapshot().expect("projection").edges().len();
    context::dispatch(
        &mut app,
        DagCommand::NodeGraphEdit(NodeGraphEdit { operations: vec![DagNodeGraphEditOp::Connect { source_node_id: source_id.clone(), source_port_id: "out".into(), target_node_id: target_id, target_port_id: "in".into() }] }),
    )
    .await;
    assert!(app.snapshot().expect("projection").edges().len() >= edges_before, "connect either adds an edge or is a safe no-op (e.g. a cycle)");
    let nodes_before = app.snapshot().expect("projection").nodes().len();
    context::dispatch(&mut app, DagCommand::NodeGraphEdit(NodeGraphEdit { operations: vec![DagNodeGraphEditOp::Delete { node_ids: vec![source_id.clone()], synapse_ids: Vec::new() }] })).await;
    let projection = app.snapshot().expect("projection");
    assert_eq!(projection.nodes().len(), nodes_before - 1);
    assert!(!projection.edges().iter().any(|edge| edge.source.starts_with(&format!("{source_id}@")) || edge.target.starts_with(&format!("{source_id}@"))), "the deleted node's wires go with it");
    context::close(&mut app);
}

/// 🧪️ A mounted wrapper never answers `InvocationResult.mutations`, so the "unknown edge is a
/// no-op" half is proved on the projected document instead of on the invocation answer.
#[semio_framework_async_macros::async_test]
async fn disconnect_removes_a_known_edge_and_is_a_no_op_for_an_unknown_one() {
    let mut app = context::new_app().await;
    let edge_id = app.snapshot().expect("projection").edges().first().map(|edge| edge.id.clone());
    if let Some(edge_id) = edge_id {
        let edges_before = app.snapshot().expect("projection").edges().len();
        context::dispatch(&mut app, DagCommand::Disconnect(disconnect::Disconnect { edge_id })).await;
        assert_eq!(app.snapshot().expect("projection").edges().len(), edges_before - 1);
    }
    let before = app.snapshot().expect("projection");
    context::dispatch(&mut app, DagCommand::Disconnect(disconnect::Disconnect { edge_id: "nonexistent".into() })).await;
    assert_eq!(app.snapshot().expect("projection"), before, "disconnecting an unknown edge leaves the document untouched");
    context::close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn connect_media_ports_adds_an_edge_between_two_nodes() {
    let mut app = context::new_app().await;
    let (source_id, target_id) = {
        let projection = app.snapshot().expect("projection");
        let nodes = projection.nodes();
        (nodes[0].id.clone(), nodes[1].id.clone())
    };
    let edges_before = app.snapshot().expect("projection").edges().len();
    context::dispatch(&mut app, DagCommand::ConnectMediaPorts(connect_media_ports::ConnectMediaPorts { source_node_id: source_id, source_port_id: "out".into(), target_node_id: target_id, target_port_id: "in".into() })).await;
    assert!(app.snapshot().expect("projection").edges().len() >= edges_before);
    context::close(&mut app);
}

//#region ✋️GestureLaws
/// 🕹️ One `nodeGraphEdit` exactly as a host sends it (`operations` plus a press's top-level `gesture`/`commit`/`abort`),
/// settled through the registered ladder; a host cancel carries no value and is settled by the runtime itself.
async fn send(app: &mut DagApp, verb: &str, args: serde_json::Value) {
    let action_meta = meta("local");
    let aborting = args.get("abort").is_some();
    let args: dsl::DslValue = args.into();
    app.handle_action(verb, Some(&args), &action_meta).await.unwrap_or_else(|fault| panic!("{verb} admitted: {fault:?}"));
    if !aborting {
        settle_registered_typed_operation(app, action_meta.instance_id).await.expect("the dispatch settles");
    }
}

/// 🧾️ Every applied history row that carries document operations, oldest first.
async fn edit_rows(app: &mut DagApp) -> Vec<HistoryEntry> {
    let mut rows: Vec<HistoryEntry> = PluginApp::history_snapshot(app).await.expect("history").upserts.into_iter().filter(|entry| entry.applied && !entry.op_lines.is_empty()).collect();
    rows.sort_by_key(|entry| entry.seq);
    rows
}

fn position(app: &DagApp, id: &str) -> (f64, f64) {
    app.snapshot().expect("projection").nodes().into_iter().find(|node| node.id == id).map(|node| (node.x, node.y)).expect("node")
}

fn slider_value(app: &DagApp, id: &str) -> f64 {
    match app.snapshot().expect("projection").nodes().into_iter().find(|node| node.id == id).map(|node| node.kind) {
        Some(DagNodeKind::Slider { value, .. }) => value,
        other => panic!("{id} is no slider: {other:?}"),
    }
}

fn english(entry: &HistoryEntry) -> String {
    entry.label.resolve(protocol::Terminology::Native, protocol::Locale::En).to_string()
}

fn german(entry: &HistoryEntry) -> String {
    entry.label.resolve(protocol::Terminology::Native, protocol::Locale::De).to_string()
}

/// ⚖️ LAW: a released node drag (the node-graph gesture record, as both hosts write it) is ONE edit, one row stamped with
/// its transaction (tool `<appId>#nodeGraphEdit`), whose op is the RELATIVE `move-nodes` leaf; every dragged node lands at
/// its base position plus the offset, and one undo puts them back.
#[semio_framework_async_macros::async_test]
async fn a_node_drag_record_is_one_transaction_of_one_relative_move() {
    let mut app = context::new_app().await;
    let ids: Vec<String> = app.snapshot().expect("projection").nodes().iter().take(2).map(|node| node.id.clone()).collect();
    let bases: Vec<(f64, f64)> = ids.iter().map(|id| position(&app, id)).collect();
    let before = edit_rows(&mut app).await.len();
    send(&mut app, "nodeGraphEdit", json!({ "operations": [{ "operation": "move", "gestureId": "node-drag:1", "nodeIds": ids, "dx": 40.0, "dy": -12.5 }] })).await;
    for (id, (x, y)) in ids.iter().zip(&bases) {
        assert_eq!(position(&app, id), (x + 40.0, y - 12.5));
    }
    let rows = edit_rows(&mut app).await;
    let rows = &rows[before..];
    assert_eq!(rows.len(), 1, "one drag, one row: {rows:?}");
    let transaction = rows[0].transaction.as_ref().expect("the row is keyed by its tool transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool == "s.dag.dag@1/*#editor#nodeGraphEdit", "{transaction:?}");
    assert_eq!(rows[0].mutations.len(), 1, "one relative leaf for the whole selection");
    assert!(rows[0].op_lines.iter().all(|line| line.starts_with("move-nodes")), "{:?}", rows[0].op_lines);
    assert_eq!(english(&rows[0]), "Move 2 node(s) by (40, -12.5)");
    assert_eq!(german(&rows[0]), "2 Knoten um (40; -12,5) verschieben");
    settle_history_verb(&mut app, "undo", meta("local").instance_id).await;
    for (id, base) in ids.iter().zip(&bases) {
        assert_eq!(position(&app, id), *base, "one undo restores the drag");
    }
    context::close(&mut app);
}

/// ⚖️ LAW: a drag that moves nothing leaves zero trace; two drags are two transactions; a palette drop
/// (`moveMediaNode`) is the same relative leaf from the node's base position, under its own tool.
#[semio_framework_async_macros::async_test]
async fn zero_trace_two_transactions_and_the_palette_drop() {
    let mut app = context::new_app().await;
    let id = app.snapshot().expect("projection").nodes()[0].id.clone();
    let (x, y) = position(&app, &id);
    let before = edit_rows(&mut app).await.len();
    send(&mut app, "nodeGraphEdit", json!({ "operations": [{ "operation": "move", "gestureId": "node-drag:0", "nodeIds": [id], "dx": 0.0, "dy": 0.0 }] })).await;
    send(&mut app, "nodeGraphEdit", json!({ "operations": [{ "operation": "move", "gestureId": "node-drag:0", "nodeIds": ["ghost"], "dx": 10.0, "dy": 0.0 }] })).await;
    assert_eq!(edit_rows(&mut app).await.len(), before, "nothing moved, nothing recorded");
    send(&mut app, "nodeGraphEdit", json!({ "operations": [{ "operation": "move", "gestureId": "node-drag:1", "nodeIds": [id], "dx": 10.0, "dy": 0.0 }] })).await;
    send(&mut app, "moveMediaNode", json!({ "nodeId": id, "x": x + 30.0, "y": y + 80.0 })).await;
    let rows = edit_rows(&mut app).await;
    let rows = &rows[before..];
    assert_eq!(rows.len(), 2, "{rows:?}");
    assert_ne!(rows[0].transaction.as_ref().expect("first").id, rows[1].transaction.as_ref().expect("second").id, "two moves are two transactions");
    assert_eq!(rows[1].transaction.as_ref().expect("drop").tool, "s.dag.dag@1/*#editor#moveMediaNode");
    assert_eq!(english(&rows[1]), "Move 1 node(s) by (20, 80)", "the drop is the offset from the base position after the first drag");
    assert_eq!(position(&app, &id), (x + 30.0, y + 80.0));
    context::close(&mut app);
}

/// ⚖️ LAW: a slider press on the canvas overlay keeps its ticks provisional and lands as ONE edit with ONE transaction of
/// the absolute `set-slider` leaf; a second press is a second transaction; a cancelled press leaves zero trace.
#[semio_framework_async_macros::async_test]
async fn a_slider_press_is_one_transaction_and_a_cancel_is_zero_trace() {
    let mut app = context::new_app().await;
    let base = slider_value(&app, "slider");
    let before = edit_rows(&mut app).await.len();
    for (value, commit) in [(6.0, false), (7.0, false), (7.5, true)] {
        send(&mut app, "nodeGraphEdit", json!({ "operations": [{ "operation": "setSlider", "widgetId": "slider", "value": value }], "gesture": "press-1", "commit": commit })).await;
        assert_eq!(slider_value(&app, "slider"), if commit { value } else { base }, "a tick stays provisional");
    }
    send(&mut app, "nodeGraphEdit", json!({ "operations": [{ "operation": "setSlider", "widgetId": "slider", "value": 2.0 }], "gesture": "press-2", "commit": true })).await;
    send(&mut app, "nodeGraphEdit", json!({ "operations": [{ "operation": "setSlider", "widgetId": "slider", "value": 9.0 }], "gesture": "press-3", "commit": false })).await;
    send(&mut app, "nodeGraphEdit", json!({ "operations": [], "gesture": "press-3", "abort": "blur" })).await;
    assert_eq!(slider_value(&app, "slider"), 2.0, "the cancelled press left no value");
    let rows = edit_rows(&mut app).await;
    let rows = &rows[before..];
    assert_eq!(rows.len(), 2, "two released presses, two rows: {rows:?}");
    assert!(rows.iter().all(|entry| entry.op_lines.iter().all(|line| line.starts_with("set-slider"))), "{rows:?}");
    let transactions: std::collections::BTreeSet<&str> = rows.iter().map(|entry| entry.transaction.as_ref().expect("every press is a transaction").id.as_str()).collect();
    assert_eq!(transactions.len(), 2);
    assert_eq!(english(&rows[0]), "Set slider \"slider\" value to 7.5");
    assert_eq!(german(&rows[0]), "Wert von Schieberegler \"slider\" auf 7,5 setzen");
    context::close(&mut app);
}

/// ⚖️ LAW: the inspector's slider fields and name field are absolute leaves — `set-slider` per addressed slider and
/// `change-node-name` per renamed node — never a whole-kind replace plus a resize.
#[semio_framework_async_macros::async_test]
async fn the_inspector_patches_are_absolute_leaves() {
    let mut app = context::new_app().await;
    let before = edit_rows(&mut app).await.len();
    context::dispatch(&mut app, DagCommand::PatchDagNodes(patch_dag_nodes::PatchDagNodes { node_ids: vec!["slider".into()], field: "max".into(), value: "20".into() })).await;
    context::dispatch(&mut app, DagCommand::PatchDagNodes(patch_dag_nodes::PatchDagNodes { node_ids: vec!["slider".into()], field: "name".into(), value: "Volume".into() })).await;
    let rows = edit_rows(&mut app).await;
    let lines: Vec<&String> = rows[before..].iter().flat_map(|entry| entry.op_lines.iter()).collect();
    assert_eq!(lines.len(), 2, "{lines:?}");
    assert!(lines[0].starts_with("set-slider") && lines[1].starts_with("change-node-name"), "{lines:?}");
    assert!(!lines.iter().any(|line| line.starts_with("replace-node-kind") || line.starts_with("resize-node")), "{lines:?}");
    context::close(&mut app);
}

async fn history_edit(app: &mut DagApp, verb: &str, args: serde_json::Value) {
    let args: dsl::DslValue = args.into();
    let result = app.handle_action(verb, Some(&args), &meta("local")).await.unwrap_or_else(|fault| panic!("{verb}: {fault:?}"));
    assert!(result.output.get("rejected").is_none(), "{verb} was refused: {:?}", result.output);
}

async fn time_travel_stage(app: &mut DagApp) -> Option<semio_framework::kernel::HistoryTimeTravelStage> {
    app.history_snapshot().await.expect("history").time_travel.map(|status| status.stage)
}

async fn pump_time_travel(app: &mut DagApp, done: impl Fn(Option<semio_framework::kernel::HistoryTimeTravelStage>) -> bool) {
    for _ in 0..10_000 {
        if done(time_travel_stage(app).await) {
            return;
        }
        app.advance_typed_operation_publication().await.expect("a driver turn");
        while app.take_typed_operation_ui_progress().is_some() {}
    }
    panic!("the history edit never settled: {:?}", time_travel_stage(app).await);
}

/// ⚖️ LAW: time travel edits the drag's offset, never the tool: superseding the first drag's `dx` replays the later drag
/// of the same node on the new base, and the overwritten head equals a fresh run of the edited drags.
#[semio_framework_async_macros::async_test]
async fn editing_a_drag_offset_in_history_replays_downstream() {
    let mut app = context::new_app().await;
    let id = app.snapshot().expect("projection").nodes()[0].id.clone();
    let (x, y) = position(&app, &id);
    send(&mut app, "nodeGraphEdit", json!({ "operations": [{ "operation": "move", "gestureId": "node-drag:1", "nodeIds": [id], "dx": 10.0, "dy": 0.0 }] })).await;
    send(&mut app, "nodeGraphEdit", json!({ "operations": [{ "operation": "move", "gestureId": "node-drag:2", "nodeIds": [id], "dx": 0.0, "dy": 7.0 }] })).await;
    let rows = edit_rows(&mut app).await;
    let first = rows[rows.len() - 2].mutations.first().map(|mutation| mutation.mutation_id.clone()).expect("the first drag's mutation");
    history_edit(&mut app, "historyEditBegin", json!({ "mutationId": first })).await;
    history_edit(&mut app, "historyEditInput", json!({ "path": "/dx", "value": 55.0 })).await;
    history_edit(&mut app, "historyEditAccept", json!({})).await;
    pump_time_travel(&mut app, |stage| stage != Some(semio_framework::kernel::HistoryTimeTravelStage::Replaying)).await;
    assert_eq!(time_travel_stage(&mut app).await, Some(semio_framework::kernel::HistoryTimeTravelStage::Reviewing));
    assert_eq!(position(&app, &id), (x + 10.0, y + 7.0), "reviewing never touches the committed document");
    history_edit(&mut app, "historyEditFinalize", json!({})).await;
    history_edit(&mut app, "historyEditCommit", json!({ "choice": "overwrite" })).await;
    pump_time_travel(&mut app, |stage| stage.is_none()).await;
    assert_eq!(position(&app, &id), (x + 55.0, y + 7.0), "the edited offset replays and the later drag still applies on top");
    let mut fresh = context::new_app().await;
    send(&mut fresh, "nodeGraphEdit", json!({ "operations": [{ "operation": "move", "gestureId": "node-drag:3", "nodeIds": [id], "dx": 55.0, "dy": 0.0 }] })).await;
    send(&mut fresh, "nodeGraphEdit", json!({ "operations": [{ "operation": "move", "gestureId": "node-drag:4", "nodeIds": [id], "dx": 0.0, "dy": 7.0 }] })).await;
    assert_eq!(app.snapshot().expect("edited head").nodes(), fresh.snapshot().expect("fresh head").nodes(), "the edited log equals a fresh run of the edited drags");
    context::close(&mut app);
    context::close(&mut fresh);
}

/// ⚖️ LAW: the host wire decodes by name — an unknown operation or a malformed gesture record is refused, never guessed.
#[test]
fn the_host_wire_decodes_by_name() {
    let decode = |value: serde_json::Value| NodeGraphEdit::from_action_args(Some(&dsl::DslValue::from(value)));
    let decoded = decode(json!({ "operations": [{ "operation": "move", "gestureId": "g", "nodeIds": ["a"], "dx": 1.0, "dy": 2.0 }, { "operation": "setSlider", "widgetId": "s", "value": 3.0 }], "gesture": "press", "commit": true })).expect("host rows decode");
    assert_eq!(decoded.operations, vec![DagNodeGraphEditOp::Move { gesture_id: "g".into(), node_ids: vec!["a".into()], dx: 1.0, dy: 2.0 }, DagNodeGraphEditOp::SetSlider { widget_id: "s".into(), value: 3.0 }]);
    assert!(decode(json!({ "operations": [{ "operation": "move", "gestureId": "g", "nodeIds": ["a"], "dx": 1.0 }] })).is_err(), "a gesture record misses dy");
    assert!(decode(json!({ "operations": [{ "operation": "teleport" }] })).is_err());
    assert_eq!(decode(json!({ "operations": [], "gesture": "press", "abort": "blur" })).expect("an abort carries no rows").operations, Vec::new());
}
/// ⚖️ LAW: the guest decodes the renderer's committed node-graph row vocabulary — every accepted row, and only those — and
/// maps each to its typed row (`insertPort` sides by name, `delete` by its node and wire ids).
#[test]
fn the_renderer_row_fixture_decodes_exactly() {
    let fixture: serde_json::Value = serde_json::from_str(NODE_GRAPH_EDIT_ROWS).expect("the row fixture parses");
    let batch = |row: &serde_json::Value| NodeGraphEdit::from_action_args(Some(&dsl::DslValue::from(json!({ "operations": [row] }))));
    for case in fixture["accepted"].as_array().expect("accepted rows") {
        assert!(batch(&case["row"]).is_ok(), "accepted row {} refused", case["id"]);
    }
    for case in fixture["refused"].as_array().expect("refused rows") {
        assert!(batch(&case["row"]).is_err(), "refused row {} decoded", case["id"]);
    }
    let decoded = NodeGraphEdit::from_action_args(Some(&dsl::DslValue::from(json!({ "operations": [
        { "operation": "insertPort", "nodeId": "add", "side": "output", "index": 0 },
        { "operation": "delete", "nodeIds": ["add"], "synapseIds": ["s1"] }
    ] })))).expect("rows decode");
    assert_eq!(decoded.operations, vec![DagNodeGraphEditOp::InsertPort { node_id: "add".into(), side: "output".into(), index: 0 }, DagNodeGraphEditOp::Delete { node_ids: vec!["add".into()], synapse_ids: vec!["s1".into()] }]);
}

/// 🧾️ The renderer's committed node-graph row vocabulary (schema `📺️renderer/🧑‍🎨engine/🧬️schema/🔣️node-graph-edit-rows`).
const NODE_GRAPH_EDIT_ROWS: &str = include_str!("../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json");
//#endregion ✋️GestureLaws

```

### ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs

SHA-256 a1812c918d0aade7ecdcfb9f0e0a55d36b2ba54b68337e08ade8a7dc58386da6; 44756 bytes.

```rust
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
    let connect = dsl::DslValue::from(serde_json::json!({ "operations": [{ "operation": "connect", "sourceNodeId": "slider", "sourcePortId": "number", "targetNodeId": "add", "targetPortId": "b" }] }));
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
    let args = dsl::DslValue::from(serde_json::json!({
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
    let malformed = dsl::DslValue::from(serde_json::json!({
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
        assert!(operations_from_action(&dsl::DslValue::from(serde_json::json!({ "operations": [refused.clone()] }))).is_err(), "{refused} is no row");
    }
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
    release_drag_as(app, &meta("renderer-node-drag"), nodes, dx, dy).await;
}

/// 🎭️ [`release_drag`] dispatched by `metadata`'s actor on `metadata`'s instance.
async fn release_drag_as(app: &mut FlowApp, metadata: &semio_framework_plugin::ActionMeta, nodes: &[&str], dx: f64, dy: f64) {
    let args = dsl::DslValue::from(serde_json::json!({ "operations": [{ "operation": "move", "gestureId": "node-drag:9", "nodeIds": nodes, "dx": dx, "dy": dy }] }));
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
async fn history_edit(app: &mut FlowApp, verb: &str, args: Vec<(&str, dsl::DslValue)>) -> dsl::DslValue {
    history_edit_as(app, &meta("history-edit"), verb, args).await
}

/// 🎭️ [`history_edit`] dispatched by `metadata`'s actor on `metadata`'s instance.
async fn history_edit_as(app: &mut FlowApp, metadata: &semio_framework_plugin::ActionMeta, verb: &str, args: Vec<(&str, dsl::DslValue)>) -> dsl::DslValue {
    let args = dsl::DslValue::Object(args.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
    app.handle_action(verb, Some(&args), metadata).await.unwrap_or_else(|fault| panic!("{verb}: {fault:?}")).output
}

/// ✏️ Edits the drag `mutation` of member `store` to `dx = 100` through time travel, accepts it and waits for the review.
async fn edit_drag_offset(app: &mut FlowApp, metadata: &semio_framework_plugin::ActionMeta, store: &str, mutation: String) {
    let begun = history_edit_as(app, metadata, "historyEditBegin", vec![("mutationId", dsl::DslValue::String(mutation)), ("store", dsl::DslValue::String(store.into()))]).await;
    assert!(begun.get("rejected").is_none(), "{begun:?}");
    let input = history_edit_as(app, metadata, "historyEditInput", vec![("path", dsl::DslValue::String("/dx".into())), ("value", dsl::DslValue::float(100.0))]).await;
    assert!(input.get("rejected").is_none(), "{input:?}");
    history_edit_as(app, metadata, "historyEditAccept", Vec::new()).await;
    pump_time_travel(app, |stage| stage != Some(semio_framework::kernel::HistoryTimeTravelStage::Replaying)).await;
    assert_eq!(time_travel_stage(app).await, Some(semio_framework::kernel::HistoryTimeTravelStage::Reviewing));
}

/// 🌿️ Finalizes the reviewed session as the new alternative `name` and waits until it closed.
async fn finalize_as_alternative(app: &mut FlowApp, metadata: &semio_framework_plugin::ActionMeta, name: &str) {
    history_edit_as(app, metadata, "historyEditFinalize", Vec::new()).await;
    let committed = history_edit_as(app, metadata, "historyEditCommit", vec![("name", dsl::DslValue::String(name.into()))]).await;
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
    assert_eq!(store, format!("content/{}", app.snapshot().expect("snapshot").content.child_id), "the parent never re-mints the live child");
    assert_eq!(published_host_snapshot(&mut app).await["layout"]["add"], serde_json::json!({ "x": 100.0, "y": 48.0 }), "the parent re-derives its scene from the replayed member");
    let edited = &member_rows(&mut app).await[0].mutations[0];
    assert!(edited.superseded && !edited.withdrawn, "the drag row shows its superseded input: {edited:?}");
    assert_eq!(edited.label.resolve(protocol::Terminology::Native, protocol::Locale::En), "Drag 1 node by (100, 48)", "the row reads the effective input");
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
    settle_registered_typed_operation(&mut *app, meta("local").instance_id).await.expect("addWidget publication");
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
const NODE_GRAPH_EDIT_ROWS_JSON: &str = include_str!("../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json");

/// ⚖️ LAW (design §13.3): the flow guest decodes every row of the shared node-graph edit vocabulary the fixture accepts
/// and refuses every row it refuses — among them the whole-fixture `setHostSnapshot` and the ambient `deleteSelection`.
#[semio_framework_async_macros::async_test]
async fn the_guest_reads_exactly_the_shared_node_graph_edit_rows() {
    let fixture: serde_json::Value = serde_json::from_str(NODE_GRAPH_EDIT_ROWS_JSON).expect("node-graph edit rows fixture");
    for case in fixture["accepted"].as_array().expect("accepted rows") {
        let args = dsl::DslValue::from(serde_json::json!({ "operations": [case["row"].clone()] }));
        assert_eq!(operations_from_action(&args).map(|rows| rows.len()).ok(), Some(1), "{} decodes", case["id"]);
    }
    for case in fixture["refused"].as_array().expect("refused rows") {
        let args = dsl::DslValue::from(serde_json::json!({ "operations": [case["row"].clone()] }));
        assert!(operations_from_action(&args).is_err(), "{} is refused", case["id"]);
    }
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


```

### ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️node-graph/🧪️tests/🔬️unit/🦀️.rs

SHA-256 300175758035d38905c814ead11831a7216066b548c371f5ecbe8db4571c5926; 8805 bytes.

```rust
use crate::editor::sequence::unit_tests::context::{dispatch, live_host_snapshot, main_window_meta, new_app, new_app_with_registry_wired};
use crate::editor::sequence::SequenceCommand;
use crate::SequenceCamera;
use semio_framework_plugin::PluginApp;

use super::set_viewport::SetViewport;

/// 🎥️ `SetViewport` is config-only — it must never emit a `SequenceMutation` (no VCS edit, no
/// undo entry) and instead write straight into the config store.
#[semio_framework_async_macros::async_test]
async fn set_viewport_writes_config_not_operations() {
    let mut app = new_app().await;
    // 🔁️ A mounted app publishes AFTER it answers, so the config write is only visible once the
    // retained operation has settled — `context::dispatch` drives that continuation the way the host
    // does. `result.mutations` is always empty on a mounted app; the no-VCS-edit claim is proven by
    // the settled receipt carrying no `Artifact` lane.
    let result = dispatch(&mut app, SequenceCommand::SetViewport(SetViewport { camera: SequenceCamera { x: 5.0, y: 6.0, zoom: 2.0 } })).await;
    assert!(result.mutations.is_empty(), "setViewport must not emit a VCS operation");
    // 🪟️ The camera lives in the MAIN window's own config, so the read has to come through the same
    // window instance the write was addressed at — an unaddressed render sees the default camera.
    let view = main_window_meta().view_state.expect("main window view");
    let node = app.render(crate::editor::sequence::modes::edit::windows::main::SEQUENCE_PLAY_BODY_MAIN, None, &view).await.expect("render");
    let semio_framework_plugin::Component::Surface(props) = &node.root.component else { panic!("semantic graph") };
    let scene: semio_framework_plugin::NodeGraphScene = semio_framework_ui_scene::decode(props).expect("packed graph");
    assert_eq!(scene.viewport.expect("camera viewport").zoom, 2.0);
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(node).expect("retire viewport graph");
}

/// 🗑️ A `delete` row removes exactly the steps it names (never an ambient selection), with every edge they hold; a row
/// outside the shared vocabulary, or one a sequence has no widget for, refuses the whole batch.
#[semio_framework_async_macros::async_test]
async fn node_graph_edit_delete_removes_the_named_step() {
    let mut app = new_app_with_registry_wired().await;
    dispatch(&mut app, SequenceCommand::NodeGraphEdit(super::node_graph_edit::NodeGraphEdit { operations_json: "[{\"operation\":\"delete\",\"nodeIds\":[\"step-1\"],\"synapseIds\":[]}]".into() })).await;
    let live = live_host_snapshot(&app).await;
    assert!(!live.steps.iter().any(|step| step.id == "step-1"));
    assert!(!live.edges.iter().any(|edge| edge.from == "step-1" || edge.to == "step-1"));
}

/// ⚖️ LAW: the sequence guest decodes the renderer's committed node-graph rows through the ONE shared decoder: every
/// refused row is refused, every accepted row a sequence carries (`move`, `connect`, `disconnect`, `delete`) decodes, and
/// the `setSlider`/`insertPort` rows it has no widget for are refused.
#[test]
fn the_renderer_row_fixture_decodes_exactly() {
    let fixture: serde_json::Value = serde_json::from_str(NODE_GRAPH_EDIT_ROWS).expect("the row fixture parses");
    for case in fixture["accepted"].as_array().expect("accepted rows") {
        let carried = !matches!(case["row"]["operation"].as_str(), Some("setSlider" | "insertPort"));
        assert_eq!(super::node_graph_edit::sequence_node_graph_row(&dsl::DslValue::from(case["row"].clone())).is_ok(), carried, "accepted row {}", case["id"]);
    }
    for case in fixture["refused"].as_array().expect("refused rows") {
        assert!(super::node_graph_edit::sequence_node_graph_row(&dsl::DslValue::from(case["row"].clone())).is_err(), "refused row {} decoded", case["id"]);
    }
}

/// 🧾️ The renderer's committed node-graph row vocabulary (schema `📺️renderer/🧑‍🎨engine/🧬️schema/🔣️node-graph-edit-rows`).
const NODE_GRAPH_EDIT_ROWS: &str = include_str!("../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json");

//#region ✋️ChildIntentLeaves
fn fold(base: &semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot, leaves: &[semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::SemioFlowMutation]) -> semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot {
    let mut state = base.clone();
    for leaf in leaves {
        let outcome = semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::apply_semio_flow_mutation(&mut state, leaf);
        assert!(outcome.worst_level().is_none_or(|level| level < protocol::Severity::Error), "{leaf:?} refused: {:?}", outcome.messages());
    }
    state
}

/// ⚖️ LAW: every editor edit publishes child INTENT leaves, never a whole `set-snapshot`, and folding them onto the base
/// content reproduces the content the editor host left behind — a drag of two steps is ONE relative `drag-nodes`.
#[semio_framework_async_macros::async_test]
async fn child_intent_leaves_reproduce_the_edited_content() {
    use crate::editor::sequence::{sequence_content_leaves, sequence_scene_leaves};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::SemioFlowMutation;
    let app = new_app().await;
    let live = live_host_snapshot(&app).await;
    let base = neural_engine::ColdOwner::new(crate::SequenceWorkingScene { steps: live.steps.clone(), edges: live.edges.clone() });
    let base_content = crate::sequence_content_snapshot_from_working(&base.steps, &base.edges);
    let ids: Vec<String> = base.steps.iter().take(2).map(|step| step.id.clone()).collect();
    let mut dragged = neural_engine::ColdOwner::new(crate::SequenceWorkingScene::clone(&base));
    for step in dragged.steps.iter_mut().filter(|step| ids.contains(&step.id)) {
        step.x += 40.0;
        step.y -= 12.5;
    }
    let leaves = sequence_scene_leaves(&base, &dragged);
    assert!(matches!(leaves.as_slice(), [SemioFlowMutation::DragNodes(drag)] if drag.targets == ids && drag.dx == 40.0 && drag.dy == -12.5), "{leaves:?}");
    let mut edited = neural_engine::ColdOwner::new(crate::SequenceWorkingScene::clone(&dragged));
    edited.edges.clear();
    if let Some(first) = edited.steps.first_mut() {
        first.kind = "log".into();
    }
    let removed = edited.steps.pop().map(neural_engine::ColdOwner::new);
    let next_content = crate::sequence_content_snapshot_from_working(&edited.steps, &edited.edges);
    let leaves = sequence_content_leaves(&base_content, &next_content);
    assert!(!leaves.iter().any(|leaf| matches!(leaf, SemioFlowMutation::SetSnapshot(_))), "{leaves:?}");
    assert_eq!(fold(&base_content, &leaves), next_content);
    assert!(sequence_content_leaves(&base_content, &base_content).is_empty(), "no change, no leaf");
    drop(removed);
}

/// ⚖️ LAW: a released node drag (the node-graph gesture record) moves its steps by the ONE offset and lands as ONE
/// composed-child tool transaction; a drag that moves nothing leaves zero trace.
#[semio_framework_async_macros::async_test]
async fn a_node_drag_record_is_one_child_transaction() {
    let mut app = new_app_with_registry_wired().await;
    let live = live_host_snapshot(&app).await;
    let (id, x, y) = live.steps.first().map(|step| (step.id.clone(), step.x, step.y)).expect("a step");
    drop(live);
    let rows = |history: semio_framework::kernel::HistoryPatch| history.upserts.into_iter().filter(|entry| entry.applied && entry.transaction.is_some()).count();
    let before = rows(app.history_snapshot().await.expect("history"));
    dispatch(&mut app, SequenceCommand::NodeGraphEdit(super::node_graph_edit::NodeGraphEdit { operations_json: format!("[{{\"operation\":\"move\",\"gestureId\":\"node-drag:0\",\"nodeIds\":[\"{id}\"],\"dx\":0.0,\"dy\":0.0}}]") })).await;
    assert_eq!(rows(app.history_snapshot().await.expect("history")), before, "a click moves nothing");
    dispatch(&mut app, SequenceCommand::NodeGraphEdit(super::node_graph_edit::NodeGraphEdit { operations_json: format!("[{{\"operation\":\"move\",\"gestureId\":\"node-drag:1\",\"nodeIds\":[\"{id}\"],\"dx\":25.0,\"dy\":5.0}}]") })).await;
    let moved = live_host_snapshot(&app).await.steps.iter().find(|entry| entry.id == id).map(|entry| (entry.x, entry.y)).expect("moved step");
    assert_eq!(moved, (x + 25.0, y + 5.0));
    assert_eq!(rows(app.history_snapshot().await.expect("history")), before + 1, "one drag, one transaction row");
}
//#endregion ✋️ChildIntentLeaves

```

### ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs

SHA-256 06270336eb69b89a65a1a94951faf0edf751e7709bf26255f6610c8d8e7d7e5c; 46799 bytes.

```rust
use super::*;
use crate::standards::v1::subsets::any::schema::Rhs;
use protocol::{OpBinary, OpText};
use semio_framework_plugin::artifact_app_laws;
use semio_framework_plugin::App;
use semio_framework_plugin::EditorApp;
use semio_framework_ui_locale::Locale;
use semio_framework_plugin::PluginApp;
use semio_framework_ui_locale::Terminology;
use semio_framework_plugin::VcsArtifactApp;
use semio_framework_plugin::ViewModel;

/// 🎫️ See `jack`'s `trinity_jack_manifest_for_tests` doc comment for why this wrapper exists
/// (SDK gap, `artifact_app_laws::new_app_with_registry`'s signature is still `fn(manifest: fn() -> App)`).
fn trinity_rewriting_manifest_for_tests() -> App {
    App { definition: create_rewriting_app(), examples: Vec::new() }
}

semio_framework_plugin::history_edit_acceptance_law!("trinity", TrinityRewritingPlayApp, trinity_rewriting_manifest_for_tests, "../..");

fn meta(actor: &str) -> semio_framework_plugin::ActionMeta {
    artifact_app_laws::meta(actor)
}

/// 🎫️ Permanent wire guard (TEMPLATE.md §7): every `TrinityRewritingCommand` variant round-trips
/// through both its binary (`OpBinary`) and text (`OpText`) codecs.
#[semio_framework_async_macros::async_test]
async fn trinity_rewriting_command_text_and_binary_round_trip() {
    let commands = vec![
        TrinityRewritingCommand::NodeGraphEdit { surface_id: "trinity.rewriting.before".into(), operations_json: "[]".into() },
        TrinityRewritingCommand::SetLhsJson { value: "{}".into() },
        TrinityRewritingCommand::SetRhsJson { value: "{}".into() },
        TrinityRewritingCommand::SetParameter { name: "label".into(), value: "hi".into() },
        TrinityRewritingCommand::AddRuleClause { kind: "where".into() },
        TrinityRewritingCommand::ResetRule,
        TrinityRewritingCommand::PatchNodes { node_ids: vec!["a".into()], field: "name".into(), value: "Renamed".into() },
        TrinityRewritingCommand::SetViewport { surface_id: Some("trinity.rewriting.before".into()), viewport: semio_framework_os_kernel::Viewport2d { x: 1.0, y: 2.0, zoom: 1.0 } },
        TrinityRewritingCommand::Reorganize,
        TrinityRewritingCommand::SetLodMode { value: "compact".into() },
    ];
    for command in commands {
        let bytes = command.encode_op().expect("encode");
        assert_eq!(TrinityRewritingCommand::decode_op(&bytes).expect("decode"), command);
        let text = command.print_op();
        assert_eq!(TrinityRewritingCommand::parse_op(&text).expect("parse"), command);
    }
}

/// 🕹️ Registry-backed (not the bare `artifact_app_laws::new_app`): `interactionSelect`/`interactionHover`
/// resolve the dispatching app's declared `AppActionRegistry.interactions`, so any test exercising
/// domain "graph" selection needs the real manifest's `.interaction(...)` declaration present.
/// 🔌️ Mounted (`bind_instance_id`): `dispatch_typed` refuses `interactive-job.live-instance` for an
/// unbound app, and the mounted app answers BEFORE its retained typed operation publishes, so a
/// dispatching test settles through `settle(&mut app)` before reading the projection.
/// 🔚 Self-closing: the store's `Drop` demands the terminal-empty witness, so the guard retires the
/// app through the framework's exact close loop (skipped while unwinding).
async fn new_app() -> RewritingTestApp {
    let mut app = artifact_app_laws::new_app_with_registry::<EditorApp<TrinityRewritingPlayApp>>(trinity_rewriting_manifest_for_tests).await;
    app.bind_instance_id(REWRITING_TEST_INSTANCE).await;
    RewritingTestApp(app)
}

const REWRITING_TEST_INSTANCE: u32 = 1;

pub(crate) struct RewritingTestApp(VcsArtifactApp<EditorApp<TrinityRewritingPlayApp>>);

impl std::ops::Deref for RewritingTestApp {
    type Target = VcsArtifactApp<EditorApp<TrinityRewritingPlayApp>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for RewritingTestApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for RewritingTestApp {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            artifact_app_laws::close_registered_fixture_app(&mut self.0);
        }
    }
}

/// 📬️ Drives the mounted app's retained typed operation to quiescence — the receipt carries the
/// lanes, effects and events the host would have seen.
async fn settle(app: &mut RewritingTestApp) -> artifact_app_laws::TypedOperationFixtureReceipt {
    artifact_app_laws::settle_registered_typed_operation(&mut app.0, REWRITING_TEST_INSTANCE).await.expect("settle the typed operation")
}

/// ↩️ Dispatches a framework-reserved history verb (`undo`/`redo`) and settles its reserved job.
async fn history(app: &mut RewritingTestApp, verb: &str) {
    artifact_app_laws::settle_history_verb(&mut app.0, verb, REWRITING_TEST_INSTANCE).await;
}

/// 🖼️ Projects a rendered body through the fixture transport — a bare `serde_json::to_string(&tree.root)`
/// fails `BuiltChildren requires retained page transport`.
async fn render(app: &mut RewritingTestApp, body_key: &str, view: &ViewModel) -> String {
    let tree = app.render(body_key, None, view).await.expect("render");
    artifact_app_laws::project_and_retire_fixture_tree(tree).expect("project semantic UI test tree")
}

/// 🕹️ Dispatches the framework-injected `interactionSelect` verb against domain "graph" — the
/// replacement for the deleted `TrinityRewritingCommand::SetSelection`. On a mounted app the verb is
/// admitted as a framework-reserved job and publishes later, so both are settled here.
async fn select_graph(app: &mut RewritingTestApp, ids: &[&str]) {
    let targets: Vec<pack::JsonValue> = ids.iter().map(|id| pack::json!({ "granularity": "node", "id": id })).collect();
    let args = pack::json_to_dsl_value(&pack::json!({ "domainId": "graph", "targets": pack::to_json_string(&targets) }));
    let admitted = app.handle_action("interactionSelect", Some(&args), &meta("local")).await.expect("interactionSelect");
    semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.0, admitted).await.expect("interactionSelect reserved-job commit");
    settle(app).await;
}

#[semio_framework_async_macros::async_test]
async fn context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last() {
    let mut app = new_app().await;
    let request = ContextMenuRequest {
        menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None },
        surface: Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: TRINITY_REWRITING_PLAY_SURFACE_BEFORE.into(),
            kind: "nodeGraph".into(),
            hits: vec![semio_framework_plugin::ContextMenuHit { domain: "node".into(), id: "n1".into(), label: None }],
            selection: vec![semio_framework_plugin::ContextMenuSelectionGroup { domain: "node".into(), ids: vec!["n1".into(), "n2".into()] }],
            text: None,
        }),
        window_instance_id: None,
        point: None,
    };
    let menu = app.context_menu(&request, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
    assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
    let last = menu.last().expect("grouped disclosure menu should not be empty");
    let last_is_destructive_leaf = last.id == "delete-selection" && last.destructive == Some(true) && last.action.as_deref() == Some("nodeGraphEdit");
    let last_is_group_ending_in_destructive = last.children.as_ref().and_then(|children| children.last()).is_some_and(|child| child.destructive == Some(true));
    assert!(last_is_destructive_leaf || last_is_group_ending_in_destructive, "known destructive delete-selection must be last: {menu:?}");
}

#[semio_framework_async_macros::async_test]
async fn renders_before_and_after_graphs() {
    let mut app = new_app().await;
    let before = render(&mut app, TRINITY_REWRITING_PLAY_BODY_BEFORE, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
    let after = render(&mut app, TRINITY_REWRITING_PLAY_BODY_AFTER, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
    assert!(before.contains("node-graph"));
    assert!(after.contains("node-graph"));
}

#[semio_framework_async_macros::async_test]
async fn compiles_jack_query_from_rule() {
    let query = compiled_jack_query(&default_rule_state());
    assert!(query.contains("MATCH"));
    assert!(query.contains("SET"));
}

#[semio_framework_async_macros::async_test]
async fn apply_rewriting_changes_after_fixture() {
    let state = default_rule_state();
    assert_ne!(state.before_fixture_json, after_fixture_json(&state));
}

#[semio_framework_async_macros::async_test]
async fn renders_lhs_rhs_graphs() {
    let mut app = new_app().await;
    let lhs_json = render(&mut app, TRINITY_REWRITING_PLAY_BODY_LHS, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
    let rhs_json = render(&mut app, TRINITY_REWRITING_PLAY_BODY_RHS, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
    assert!(lhs_json.contains("node-graph"));
    assert!(rhs_json.contains("node-graph"));
    let lhs = artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::NodeGraphScene>(&lhs_json).expect("lhs node-graph scene");
    let rhs = artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::NodeGraphScene>(&rhs_json).expect("rhs node-graph scene");
    assert_eq!(lhs.editable, Some(true));
    assert_eq!(rhs.editable, Some(true));
}

#[semio_framework_async_macros::async_test]
async fn set_parameter_emits_one_op_and_is_undoable() {
    let mut app = new_app().await;
    let history_before = app.history_snapshot().await.expect("history").upserts.len();
    app.dispatch_typed(TrinityRewritingCommand::SetParameter { name: "label".into(), value: "changed".into() }, &meta("local")).await.expect("set parameter");
    let receipt = settle(&mut app).await;
    assert!(receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Artifact), "a parameter edit publishes on the artifact lane: {:?}", receipt.lanes);
    assert_eq!(app.history_snapshot().await.expect("history").upserts.len(), history_before + 1, "a single-key parameter edit is one ChangeParameterBinding operation (one history entry)");
    assert_eq!(app.snapshot().unwrap().parameter_bindings.get("label").cloned(), Some(PropertyValue::String("changed".into())));
    history(&mut app, "undo").await;
    assert_eq!(app.snapshot().unwrap().parameter_bindings.get("label").cloned(), Some(PropertyValue::String("nakagin-core".into())));
}

#[semio_framework_async_macros::async_test]
async fn add_and_delete_rhs_set_clause() {
    let mut app = new_app().await;
    app.dispatch_typed(TrinityRewritingCommand::AddRuleClause { kind: "set".into() }, &meta("local")).await.expect("add clause");
    settle(&mut app).await;
    let rhs: Rhs = pack::from_json_str(&app.snapshot().unwrap().rhs_json).unwrap();
    assert_eq!(rhs.set.len(), 2);
    let result = app
        .dispatch_typed(TrinityRewritingCommand::NodeGraphEdit { surface_id: TRINITY_REWRITING_PLAY_SURFACE_RHS.into(), operations_json: pack::json!([{ "operation": "delete", "nodeIds": ["rhs-set-1"], "synapseIds": [] }]).to_string() }, &meta("local"))
        .await
        .expect("delete the clause node");
    let receipt = settle(&mut app).await;
    assert!(!result.mutations.is_empty() || receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Artifact), "a delete row must publish an artifact mutation: {:?}", receipt.lanes);
    let rhs: Rhs = pack::from_json_str(&app.snapshot().unwrap().rhs_json).unwrap();
    assert_eq!(rhs.set.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn jack_view_renders_compiled_query_tokens() {
    let mut app = new_app().await;
    let json = render(&mut app, TRINITY_REWRITING_PLAY_BODY_JACK, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
    let scene = artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::TextEditorScene>(&json).expect("text-editor scene");
    assert!(scene.tokens_json.is_some_and(|tokens| !tokens.is_empty()));
}

#[semio_framework_async_macros::async_test]
async fn graph_scenes_have_lod_json() {
    let mut app = new_app().await;
    let json = render(&mut app, TRINITY_REWRITING_PLAY_BODY_BEFORE, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
    let scene = artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::NodeGraphScene>(&json).expect("node-graph scene");
    assert!(scene.lod_json.is_some(), "lodJson lane missing: {json}");
}

#[semio_framework_async_macros::async_test]
async fn app_definition_declares_reorganize_and_history_actions() {
    let definition = create_rewriting_app();
    let action_ids: Vec<&str> = definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).map(|action| action.id.as_str()).collect();
    assert!(action_ids.contains(&"undo"));
    assert!(action_ids.contains(&"reorganize"));
}

/// ⚖️ LAW: the navbar example picker's verb reaches EVERY window kind. `setActiveExample` is
/// app-scoped — no `window_kind_action_refs` owns it — so `try_build_definition` leaves it on the
/// app roster (`AppDefinition.actions`), which is what makes the shell's boot dispatch dispatchable
/// from whichever pane happens to be focused: `undeclaredActionDiagnostic` accepts an action found
/// on a window kind OR on the app itself (`🧱️elements/🛠️ShellHelpers/🟦️.tsx`). It was declared
/// nowhere at all, so the very first dispatch of every boot was dropped `undeclared-action` from
/// `trinity-rewriting-lhs` (ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END,
/// `📓️b3b-trinity-wfc-puzzle.md` §3.1).
#[semio_framework_async_macros::async_test]
async fn app_definition_declares_set_active_example_unscoped_for_every_window_kind() {
    let definition = create_rewriting_app();
    let scoped = definition.window_kinds.iter().filter(|window| window.actions.iter().any(|action| action.id == "setActiveExample")).count();
    assert!(definition.actions.iter().any(|action| action.id == "setActiveExample"), "setActiveExample must sit on the app roster so every window kind can dispatch it");
    assert_eq!(scoped, 0, "scoping setActiveExample to one window kind would take it off the app roster and strand the panes that do not own it");
}

/// ⚖️ LAW: `setActiveExample` answers the id the SHELL sends with a whole-document `LoadDocument`
/// effect. The navbar picker dispatches a REGISTERED example id, and `demo` is the only example this
/// subset registers, so that id must resolve. The effect is HOST-applied — the guest's own snapshot
/// is deliberately left alone here (asserting it changed is what a first draft of this test got
/// wrong), so the guest-side contract is exactly "one `LoadDocument` carrying the example's document".
#[semio_framework_async_macros::async_test]
async fn set_active_example_loads_the_registered_demo_document() {
    let mut app = new_app().await;
    app.dispatch_typed(TrinityRewritingCommand::SetActiveExample { example_id: crate::examples::demo::ID.into() }, &meta("local")).await.expect("set active example");
    let receipt = settle(&mut app).await;
    let loads = receipt.effects.iter().filter(|effect| matches!(effect, semio_framework_plugin::Effect::LoadDocument { .. })).count();
    assert_eq!(loads, 1, "the registered example id produces exactly one LoadDocument effect");
    let loaded = crate::editor::rewriting::commands::set_active_example_document(crate::examples::demo::ID).expect("the demo example resolves to a document");
    assert_eq!(loaded, store::ArtifactDsl::parse_dsl(crate::examples::demo::PRIMARY_TEXT).expect("the demo example's own dsl parses"), "the document the effect carries is the example the subset registers");
    assert_ne!(loaded, app.snapshot().unwrap(), "the effect is HOST-applied: a mounted app with no host keeps its own snapshot, which is what makes the effect the only guest-side witness");
}

#[semio_framework_async_macros::async_test]
async fn set_active_example_ignores_an_unregistered_id() {
    let mut app = new_app().await;
    app.dispatch_typed(TrinityRewritingCommand::SetActiveExample { example_id: "not-an-example".into() }, &meta("local")).await.expect("set active example");
    let receipt = settle(&mut app).await;
    assert!(receipt.effects.is_empty(), "an unknown example id loads nothing");
}

/// ⚖️ LAW: the verb crosses the SAME two gates every other rewriting document verb crosses — the
/// binary tool-job roster (`TOOL_JOB_IDS`, the wire contract) and the retained document-tool roster
/// (`REWRITING_DOCUMENT_TOOL_IDS`, which the factory keys on). Declaring the action without both
/// leaves it dispatchable but unroutable.
#[semio_framework_async_macros::async_test]
async fn set_active_example_is_registered_on_both_tool_rosters() {
    assert!(<TrinityRewritingCommand as OpBinary>::TOOL_JOB_IDS.contains(&"setActiveExample"));
    assert!(REWRITING_DOCUMENT_TOOL_IDS.contains(&"setActiveExample"));
}

#[semio_framework_async_macros::async_test]
async fn trinity_rewriting_labels_resolve_native_by_default() {
    let mut app = new_app().await;
    let json = render(&mut app, TRINITY_REWRITING_PLAY_BODY_ARTIFACT, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
    assert!(json.contains("\"Pieces\""));
    assert!(!json.contains("Stücke"));
}

#[semio_framework_async_macros::async_test]
async fn trinity_rewriting_labels_translate_panels_in_german() {
    let mut app = new_app().await;
    let view = ViewModel { locale: Locale::De, terminology: Terminology::Native, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let document_json = render(&mut app, TRINITY_REWRITING_PLAY_BODY_ARTIFACT, &view).await;
    assert!(document_json.contains("Stücke"));
    assert!(!document_json.contains("\"Pieces\""));
    let catalogue_json = render(&mut app, TRINITY_REWRITING_PLAY_BODY_CATALOGUE, &view).await;
    assert!(catalogue_json.contains("Katalog"));
    assert!(catalogue_json.contains("Zu LHS hinzufügen"));
    assert!(catalogue_json.contains("Zu RHS hinzufügen"));
    let parameters_json = render(&mut app, TRINITY_REWRITING_PLAY_BODY_PARAMETERS, &view).await;
    assert!(parameters_json.contains("\"Parameter\""));
    let definition = create_rewriting_app();
    let reset_rule = definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).find(|action| action.id == "resetRule").expect("resetRule action");
    assert_eq!(reset_rule.label.resolve(Terminology::Native, Locale::De), "Regel zurücksetzen");
}

#[semio_framework_async_macros::async_test]
async fn set_lhs_json_undo_redo_round_trip() {
    let mut app = new_app().await;
    let original = app.snapshot().unwrap().lhs_json;
    let next_lhs = r#"{"pattern":{"leftVar":"x","leftKind":"Piece","edgeVar":"r","edgeKind":"Connection","rightVar":"y","rightKind":"Piece"}}"#;
    app.dispatch_typed(TrinityRewritingCommand::SetLhsJson { value: next_lhs.into() }, &meta("local")).await.expect("set lhs");
    settle(&mut app).await;
    assert_eq!(app.snapshot().unwrap().lhs_json, next_lhs);
    history(&mut app, "undo").await;
    assert_eq!(app.snapshot().unwrap().lhs_json, original);
    history(&mut app, "redo").await;
    assert_eq!(app.snapshot().unwrap().lhs_json, next_lhs);
}

#[semio_framework_async_macros::async_test]
async fn export_media_graph_out_reflects_rule_applied_fixture() {
    let mut app = new_app().await;
    let graph_out = app.export_media("graph:out").await.expect("graph:out export");
    let MediaPayload::Structured { json, .. } = graph_out.payload else { panic!("structured payload") };
    let bytes = store::pack_rt::pack_value_from_base64(&json).expect("decode base64");
    let fixture = <JackSnapshot as ArtifactPack>::decode_pack(&bytes).expect("decode pack");
    let expected = JackSnapshot::from_json(&after_fixture_json(&app.snapshot().unwrap())).unwrap();
    assert_eq!(fixture.nodes().len(), expected.nodes().len());
}

#[semio_framework_async_macros::async_test]
async fn rewriting_io_declares_graph_in_and_graph_out_ports() {
    let io = rewriting_io();
    assert_eq!(io.artifact_schema, REWRITE_RULE_SCHEMA);
    let graph_in = io.ports.iter().find(|port| port.id == "graph:in").expect("graph:in declared");
    assert_eq!(graph_in.kind_id.as_deref(), Some("graph.trinity"));
    assert_eq!(graph_in.multiplicity, semio_framework_plugin::PortMultiplicity::One);
    let graph_out = io.ports.iter().find(|port| port.id == "graph:out").expect("graph:out declared");
    assert_eq!(graph_out.multiplicity, semio_framework_plugin::PortMultiplicity::Many);
}

#[semio_framework_async_macros::async_test]
async fn reset_document_ownership_rewriting_preserves_pack_with_an_edit_free_history() {
    use store::ArtifactPack;
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/♻️reset-document.json")).unwrap();
    let source = RewritingSnapshot { before_fixture_json: "{}".into(), lhs_json: "{}".into(), rhs_json: "{}".into(), parameter_bindings: BTreeMap::new(), rule_layout: BTreeMap::new() };
    let before = serde_json::from_str::<serde_json::Value>(&dsl::os_pack::json::to_json_string(&source)).unwrap();
    let semio_framework_plugin::Effect::LoadDocument { pack, spr } = reset_document_effect(&source) else { panic!("reset must load a document"); };
    let decoded = <RewritingSnapshot as ArtifactPack>::decode_pack(&pack).unwrap();
    assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::os_pack::json::to_json_string(&decoded)).unwrap(), before);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::os_pack::json::to_json_string(&source)).unwrap(), before);
    let history = store::os_spr::decode_history(&spr, &store::os_spr::DecodeOptions::default()).await.unwrap();
    let actual = serde_json::json!({ "documentId": history.doc_id, "schema": history.schema, "edits": history.edits.len(), "transitions": history.transitions.len(), "conflicts": history.conflicts.len() });
    assert_eq!(actual, expected);
}

//#region 🩹️RailVerbLaws
fn working_graph_node_ids(state: &RewritingSnapshot) -> Vec<String> {
    semio_s_artifact_trinity_jack::JackSnapshot::from_json(&state.before_fixture_json).expect("the working graph decodes").nodes().into_iter().map(|node| node.id).collect()
}

fn working_graph_node_name(state: &RewritingSnapshot, id: &str) -> String {
    semio_s_artifact_trinity_jack::JackSnapshot::from_json(&state.before_fixture_json).expect("the working graph decodes").nodes().into_iter().find(|node| node.id == id).map(|node| node.name).expect("node")
}

/// ⚖️ LAW: `patchNodes` pressed from the rail with an EMPTY `nodeIds` patches the selected nodes of the
/// working graph, and a comma list in the text field names several nodes (S15: the verb "moved nothing").
#[semio_framework_async_macros::async_test]
async fn patch_nodes_from_the_rail_patches_the_selection_or_the_listed_nodes() {
    let mut app = new_app().await;
    let ids = working_graph_node_ids(&app.snapshot().expect("projection"));
    select_graph(&mut app, &[&ids[0]]).await;
    let rail = |pairs: &[(&str, &str)]| pack::json_to_dsl_value(&pack::JsonValue::Object(pairs.iter().map(|(key, value)| ((*key).to_string(), pack::JsonValue::String((*value).to_string()))).collect()));
    app.handle_action("patchNodes", Some(&rail(&[("field", "name"), ("value", "S15 Selected")])), &meta("local")).await.expect("an empty nodeIds patches the selection");
    settle(&mut app).await;
    assert_eq!(working_graph_node_name(&app.snapshot().expect("projection"), &ids[0]), "S15 Selected");
    app.handle_action("patchNodes", Some(&rail(&[("nodeIds", &format!("{} {}", ids[0], ids[1])), ("field", "name"), ("value", "S15 Listed")])), &meta("local")).await.expect("a listed nodeIds patches both nodes");
    settle(&mut app).await;
    let state = app.snapshot().expect("projection");
    assert_eq!((working_graph_node_name(&state, &ids[0]), working_graph_node_name(&state, &ids[1])), ("S15 Listed".to_string(), "S15 Listed".to_string()));
}

/// ⚖️ LAW: a `patchNodes` that cannot move the document is refused by name — an unknown id is
/// `mutation.target-missing`, no id and no selection is `app.command.targets-required` (the precondition an agent,
/// which has no selection, meets by naming `nodeIds`), an unsupported field or an empty value is
/// `app.command.invalid-args`. It used to answer an empty emit that read as an accepted edit.
#[semio_framework_async_macros::async_test]
async fn patch_nodes_refuses_what_it_cannot_apply() {
    let state = default_rule_state();
    let first = working_graph_node_ids(&state)[0].clone();
    let code = |result: Result<Emit<RewriteRuleMutation, NoConfigMutation>, Fault>| result.err().expect("refused").code.0;
    assert_eq!(code(crate::editor::rewriting::commands::patch_nodes(&state, &["no-such-node".into()], &[], "name", "x")), "mutation.target-missing");
    assert_eq!(code(crate::editor::rewriting::commands::patch_nodes(&state, &[], &[], "name", "x")), "app.command.targets-required");
    assert_eq!(code(crate::editor::rewriting::commands::patch_nodes(&state, &[first.clone()], &[], "colour", "x")), "app.command.invalid-args");
    assert_eq!(code(crate::editor::rewriting::commands::patch_nodes(&state, &[first.clone()], &[], "kind", " ")), "app.command.invalid-args");
    assert_eq!(code(crate::editor::rewriting::commands::patch_nodes(&state, &[first.clone()], &[], "kind", "NoSuchKind")), "app.command.invalid-args", "a kind the manifest does not declare is refused, not written");
    assert!(!crate::editor::rewriting::commands::patch_nodes(&state, &[], &[first], "name", "Beam").expect("the selection is the target").artifact_mutations.is_empty());
}

/// 🚫️ LAW: `addRuleClause` refuses what it cannot add — a second WHERE clause, an unknown clause kind, a rule whose
/// own JSON no longer decodes — instead of an empty emit (S15: `kind=where` on the default rule journalled a row and
/// changed nothing).
#[semio_framework_async_macros::async_test]
async fn add_rule_clause_refuses_what_it_cannot_add() {
    let state = default_rule_state();
    let add = crate::editor::rewriting::commands::add_rule_clause_command;
    let code = |result: Result<Emit<RewriteRuleMutation, NoConfigMutation>, Fault>| result.err().expect("refused").code.0;
    assert_eq!(code(add(&state, "where")), "app.command.invalid-args", "the default rule already has a WHERE clause");
    assert_eq!(code(add(&state, "optional")), "app.command.invalid-args");
    let broken = RewritingSnapshot { lhs_json: "{".into(), ..state.clone() };
    assert_eq!(code(add(&broken, "create")), "trinity.rewriting.rule-undecodable");
}

/// ⏪️ LAW: a rail `addRuleClause kind=create` is ONE undoable edit that rewrites only the RHS — undo restores the
/// rule and redo adds the clause again (S15, session 12: its row carried a re-printed LHS and undo did nothing).
#[semio_framework_async_macros::async_test]
async fn a_rail_add_rule_clause_rewrites_only_the_rhs_and_round_trips() {
    let mut app = new_app().await;
    let before = app.snapshot().expect("projection");
    let emit = crate::editor::rewriting::commands::add_rule_clause_command(&before, "create").expect("create clause");
    assert!(!emit.artifact_mutations.is_empty() && emit.artifact_mutations.iter().all(|mutation| matches!(mutation, RewriteRuleMutation::EditRhs(_))), "only the RHS changes: {:?}", emit.artifact_mutations);
    let rail = pack::json_to_dsl_value(&pack::json!({ "kind": "create" }));
    app.handle_action("addRuleClause", Some(&rail), &meta("local")).await.expect("addRuleClause");
    settle(&mut app).await;
    let after = app.snapshot().expect("projection");
    assert_ne!(after.rhs_json, before.rhs_json, "the clause is added");
    assert_eq!(after.lhs_json, before.lhs_json, "the LHS is untouched");
    history(&mut app, "undo").await;
    assert_eq!(app.snapshot().expect("projection").rhs_json, before.rhs_json, "undo restores the RHS");
    history(&mut app, "redo").await;
    assert_eq!(app.snapshot().expect("projection").rhs_json, after.rhs_json, "redo adds the clause again");
}

/// ⚖️ LAW: `nodeGraphEdit` is the node-graph host's gesture verb (a `surfaceId` plus an `operations`
/// list), so it stays out of the palette, the Actions rail and the context menu's `transform` group —
/// pressed there it was refused `missing operationsJson` (S15). The menu's own delete row still
/// dispatches it with its full payload.
#[semio_framework_async_macros::async_test]
async fn the_node_graph_gesture_verb_is_kept_off_the_rail_and_the_transform_group() {
    let definition = create_rewriting_app();
    let edit = definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).find(|action| action.id == "nodeGraphEdit").expect("declared");
    assert!(!edit.in_palette, "nodeGraphEdit is a canvas gesture, never a rail row");
    let patch = definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).find(|action| action.id == "patchNodes").expect("declared");
    assert!(patch.args.iter().any(|arg| arg.id == "nodeIds" && !arg.required), "nodeIds is optional: empty means the selection");
    let mut app = new_app().await;
    let request = ContextMenuRequest {
        menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None },
        surface: Some(semio_framework_plugin::ContextMenuSurfaceTarget { surface_id: TRINITY_REWRITING_PLAY_SURFACE_BEFORE.into(), kind: "nodeGraph".into(), hits: Vec::new(), selection: Vec::new(), text: None }),
        window_instance_id: None,
        point: None,
    };
    let menu = app.context_menu(&request, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
    let bare_edit_rows = menu.iter().flat_map(|row| std::iter::once(row).chain(row.children.iter().flatten())).filter(|row| row.action.as_deref() == Some("nodeGraphEdit") && row.id != "delete-selection").count();
    assert_eq!(bare_edit_rows, 0, "no argument-less nodeGraphEdit row: {menu:?}");
}
//#endregion 🩹️RailVerbLaws

/// 🎯️ LAW: the editor declares the artifact kind it edits (the artifact's own `artifact_kind()`), which is
/// what the hub's one open-target rule (`app_opens_kind`, `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs`)
/// pairs with this editor and the viewer of its dialect — so a Trinity rewrite rule can be created and opened as a hub document.
#[semio_framework_async_macros::async_test]
async fn the_editor_declares_the_artifact_kind_it_edits() {
    assert_eq!(create_rewriting_app().artifact_kinds, vec![crate::artifact_kind()]);
}

//#region 🕹️NodeDragLaws
fn working_graph_positions(state: &RewritingSnapshot) -> Vec<(String, f64, f64)> {
    semio_s_artifact_trinity_jack::JackSnapshot::from_json(&state.before_fixture_json).expect("the working graph decodes").nodes().into_iter().map(|node| (node.id, node.x, node.y)).collect()
}

fn drag_row(gesture: &str, node_ids: &[&str], dx: f64, dy: f64) -> pack::JsonValue {
    pack::json!({ "operation": "move", "gestureId": gesture, "nodeIds": node_ids, "dx": dx, "dy": dy })
}

fn applied(state: &RewritingSnapshot, leaves: &[RewriteRuleMutation]) -> RewritingSnapshot {
    let mut next = state.clone();
    for leaf in leaves {
        crate::apply_rewrite_rule_mutation(&mut next, leaf).expect("the leaf applies");
    }
    next
}

/// ⚖️ LAW: a released working-graph drag (the hosts' `move` gesture record) is ONE tool transaction of `<appId>#nodeGraphEdit`
/// holding ONE relative `drag-working-nodes {targets, dx, dy}` — every target moves by the offset, every other node stays — and
/// ONE inverse row restores the graph; two gestures are two transactions, a seedless view publishes the leaf plainly, and a
/// release that moves nothing leaves zero trace.
#[test]
fn a_released_working_graph_drag_is_one_tool_transaction_of_one_relative_leaf() {
    let state = default_rule_state();
    let ids = working_graph_node_ids(&state);
    let (first, second) = (ids[0].as_str(), ids[1].as_str());
    let operations = pack::JsonValue::Array(vec![drag_row("g-1", &[first, second], 24.0, -8.0)]).to_string();
    let emit = crate::editor::rewriting::commands::node_graph_edit(&state, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, &operations, "seed").expect("a drag row is admitted");
    let transaction = emit.transaction.clone().expect("the release is a tool transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool == "s.trinity.rewriting@1/*#editor#nodeGraphEdit", "{transaction:?}");
    assert!(emit.coalesce_key.is_none(), "a committed transaction is a plain edit");
    assert!(matches!(emit.artifact_mutations.as_slice(), [RewriteRuleMutation::DragWorkingNodes(leaf)] if leaf.targets == vec![first.to_string(), second.to_string()] && (leaf.dx, leaf.dy) == (24.0, -8.0)), "{:?}", emit.artifact_mutations);
    let moved = applied(&state, &emit.artifact_mutations);
    for ((id, x, y), (_, before_x, before_y)) in working_graph_positions(&moved).into_iter().zip(working_graph_positions(&state)) {
        let expected = if id == first || id == second { (before_x + 24.0, before_y - 8.0) } else { (before_x, before_y) };
        assert_eq!((x, y), expected, "node {id}");
    }
    let inverse = crate::inverse_rewrite_rule_mutation(&state, &emit.artifact_mutations[0]);
    assert!(matches!(inverse.as_slice(), [RewriteRuleMutation::EditBeforeFixture(_)]), "one inverse row: {inverse:?}");
    assert_eq!(applied(&moved, &inverse), state, "the inverse row restores the working graph");
    let again = crate::editor::rewriting::commands::node_graph_edit(&state, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, &pack::JsonValue::Array(vec![drag_row("g-2", &[first], 1.0, 0.0)]).to_string(), "seed").expect("a second drag");
    assert_ne!(again.transaction.expect("second ref").id, transaction.id, "two gestures are two transactions");
    let plain = crate::editor::rewriting::commands::node_graph_edit(&state, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, &operations, "").expect("a seedless drag");
    assert!(plain.transaction.is_none() && plain.artifact_mutations.len() == 1, "a view without command authority publishes the leaf plainly");
    let idle = crate::editor::rewriting::commands::node_graph_edit(&state, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, &pack::JsonValue::Array(vec![drag_row("g-3", &[first], 0.0, 0.0)]).to_string(), "seed").expect("a zero drag");
    assert!(idle.artifact_mutations.is_empty() && idle.transaction.is_none(), "a release that moved nothing leaves zero trace");
    let malformed = crate::editor::rewriting::commands::node_graph_edit(&state, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, &pack::json!([{ "operation": "move", "gestureId": "g", "nodeIds": [], "dx": 1.0, "dy": 0.0 }]).to_string(), "seed");
    assert!(malformed.is_err_and(|fault| fault.code.0 == "trinity.rewriting.node-graph.row"), "a malformed gesture record is refused by name");
}

/// ⚖️ LAW: a rule-node drag on the LHS/RHS canvas is ONE relative `drag-rule-nodes` that moves each clause from where it sits —
/// its layout point, else its default slot — and ONE `set-rule-layout-points` row undoes it exactly (a node that had no point
/// is cleared back to its slot).
#[test]
fn a_rule_node_drag_moves_each_clause_from_where_it_sits_and_undoes_in_one_row() {
    let mut state = default_rule_state();
    state.rule_layout.insert("lhs-where".into(), LayoutPoint { x: 300.0, y: 90.0 });
    let operations = pack::JsonValue::Array(vec![drag_row("g-lhs", &["lhs-match", "lhs-where"], 30.0, 10.0)]).to_string();
    let emit = crate::editor::rewriting::commands::node_graph_edit(&state, TRINITY_REWRITING_PLAY_SURFACE_LHS, &operations, "seed").expect("a rule-node drag");
    assert!(matches!(emit.artifact_mutations.as_slice(), [RewriteRuleMutation::DragRuleNodes(_)]), "{:?}", emit.artifact_mutations);
    let moved = applied(&state, &emit.artifact_mutations);
    assert_eq!(moved.rule_layout.get("lhs-match"), Some(&LayoutPoint { x: 30.0, y: 10.0 }), "the match moves from its default slot");
    assert_eq!(moved.rule_layout.get("lhs-where"), Some(&LayoutPoint { x: 330.0, y: 100.0 }), "the WHERE clause moves from its layout point");
    let inverse = crate::inverse_rewrite_rule_mutation(&state, &emit.artifact_mutations[0]);
    assert!(matches!(inverse.as_slice(), [RewriteRuleMutation::SetRuleLayoutPoints(undo)] if undo.cleared == vec!["lhs-match".to_string()] && undo.points.len() == 1), "one exact inverse row: {inverse:?}");
    assert_eq!(applied(&moved, &inverse), state);
}

/// ⚖️ LAW (fixture `🧫️fixtures/🧫️node-graph-edit-rows`): every accepted row of the shared node-graph record vocabulary maps to an
/// intent leaf on the working graph — `connect` draws ONE `connect-working-ports` carrying the graph's edge kind, `disconnect` cuts
/// ONE `disconnect-working-edges` — every refused row refuses the whole batch by name, `setSlider`/`insertPort` are refused (the
/// graph has neither), and a rule side draws or cuts no wire alone.
#[test]
fn the_shared_node_graph_rows_map_to_intent_leaves() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json")).expect("node-graph-edit-rows fixture");
    let state = default_rule_state();
    let edit = |surface: &str, rows: serde_json::Value| crate::editor::rewriting::commands::node_graph_edit(&state, surface, &rows.to_string(), "seed");
    for refused in corpus["refused"].as_array().expect("refused rows") {
        let emit = edit(TRINITY_REWRITING_PLAY_SURFACE_BEFORE, serde_json::json!([refused["row"]]));
        assert!(emit.is_err_and(|fault| fault.code.0 == "trinity.rewriting.node-graph.row"), "refused row {} is refused by name", refused["id"]);
    }
    let graph = semio_s_artifact_trinity_jack::JackSnapshot::from_json(&state.before_fixture_json).expect("the working graph decodes");
    let (nodes, edges) = (graph.nodes(), graph.edges());
    let connect = serde_json::json!([{ "operation": "connect", "sourceNodeId": nodes[0].id, "sourcePortId": "out", "targetNodeId": nodes[1].id, "targetPortId": "in" }]);
    let drawn = edit(TRINITY_REWRITING_PLAY_SURFACE_BEFORE, connect.clone()).expect("a connect row");
    let (source, target) = (format!("{}@out", nodes[0].id), format!("{}@in", nodes[1].id));
    assert!(drawn.transaction.is_none() && matches!(drawn.artifact_mutations.as_slice(), [RewriteRuleMutation::ConnectWorkingPorts(leaf)] if leaf.source == source && leaf.target == target && leaf.kind == edges[0].kind), "{:?}", drawn.artifact_mutations);
    let wired = applied(&state, &drawn.artifact_mutations);
    let after = semio_s_artifact_trinity_jack::JackSnapshot::from_json(&wired.before_fixture_json).expect("the wired graph decodes");
    assert_eq!(after.edges().len(), edges.len() + 1, "one wire is drawn");
    assert!(after.edges().iter().any(|edge| edge.source == source && edge.target == target && edge.id == format!("{source}->{target}")));
    let inverse = crate::inverse_rewrite_rule_mutation(&state, &drawn.artifact_mutations[0]);
    assert!(matches!(inverse.as_slice(), [RewriteRuleMutation::EditBeforeFixture(_)]), "one inverse row: {inverse:?}");
    assert_eq!(applied(&wired, &inverse), state, "the inverse row restores the working graph");
    let cut = edit(TRINITY_REWRITING_PLAY_SURFACE_BEFORE, serde_json::json!([{ "operation": "disconnect", "synapseId": edges[0].id }])).expect("a disconnect row");
    assert!(matches!(cut.artifact_mutations.as_slice(), [RewriteRuleMutation::DisconnectWorkingEdges(leaf)] if leaf.targets == vec![edges[0].id.clone()]), "{:?}", cut.artifact_mutations);
    let severed = semio_s_artifact_trinity_jack::JackSnapshot::from_json(&applied(&state, &cut.artifact_mutations).before_fixture_json).expect("the cut graph decodes");
    assert_eq!((severed.nodes().len(), severed.edges().len()), (nodes.len(), edges.len() - 1), "one wire is cut, every node stays");
    for row in [serde_json::json!({ "operation": "setSlider", "widgetId": "w", "value": 1.0 }), serde_json::json!({ "operation": "insertPort", "nodeId": nodes[0].id, "side": "input", "index": 0 })] {
        assert!(edit(TRINITY_REWRITING_PLAY_SURFACE_BEFORE, serde_json::json!([row])).is_err_and(|fault| fault.code.0 == "trinity.rewriting.node-graph.row"), "{row} is refused by name");
    }
    assert!(edit(TRINITY_REWRITING_PLAY_SURFACE_LHS, connect).is_err_and(|fault| fault.code.0 == "trinity.rewriting.node-graph.row"), "a rule side draws no wire alone");
}

/// ⚖️ LAW: one canvas drag through the shell is ONE edit and ONE history row labelled from its leaf in every language, and ONE
/// undo moves the nodes back.
#[semio_framework_async_macros::async_test]
async fn one_canvas_drag_is_one_history_row_labelled_from_its_leaf() {
    use semio_framework_plugin::PluginApp;
    let mut app = new_app().await;
    let before = app.snapshot().expect("projection");
    let ids = working_graph_node_ids(&before);
    let rows_before = app.history_snapshot().await.expect("history").upserts.into_iter().filter(|row| row.edit_id.is_some()).count();
    let operations = pack::JsonValue::Array(vec![drag_row("g-shell", &[ids[0].as_str(), ids[1].as_str()], 24.0, -8.0)]).to_string();
    let args = pack::json_to_dsl_value(&pack::json!({ "surfaceId": TRINITY_REWRITING_PLAY_SURFACE_BEFORE, "operations": operations }));
    app.handle_action("nodeGraphEdit", Some(&args), &meta("local")).await.expect("the drag is admitted");
    settle(&mut app).await;
    let rows: Vec<_> = app.history_snapshot().await.expect("history").upserts.into_iter().filter(|row| row.edit_id.is_some()).collect();
    assert_eq!(rows.len(), rows_before + 1, "one drag, one edit, one row");
    let row = rows.iter().max_by_key(|row| row.seq).expect("the drag's row");
    assert_eq!(row.mutations.len(), 1, "one relative leaf");
    assert_eq!(row.mutations[0].label.resolve(Terminology::Native, Locale::En), "Drag 2 nodes by (24, -8)");
    assert_eq!(row.mutations[0].label.resolve(Terminology::Native, Locale::De), "2 Knoten um (24; -8) ziehen");
    history(&mut app, "undo").await;
    assert_eq!(working_graph_positions(&app.snapshot().expect("projection")), working_graph_positions(&before), "one undo moves the nodes back");
}

/// ⚖️ LAW: a `delete` row on the working graph is ONE relative `delete-working-nodes` of its nodes — every edge touching them goes
/// with them — followed by ONE `disconnect-working-edges` of the wires it names apart from those; each undoes with ONE row, and a
/// node the graph does not hold is skipped (`mutation.partial`), never a whole-graph write.
#[test]
fn a_deleted_working_graph_selection_is_relative_leaves() {
    use semio_s_artifact_trinity_jack::port_node_id;
    let state = default_rule_state();
    let graph = semio_s_artifact_trinity_jack::JackSnapshot::from_json(&state.before_fixture_json).expect("the working graph decodes");
    let touches = |edge: &semio_s_artifact_trinity_jack::Edge, id: &str| port_node_id(&edge.source).unwrap_or(&edge.source) == id || port_node_id(&edge.target).unwrap_or(&edge.target) == id;
    let doomed = graph.nodes().into_iter().map(|node| node.id).find(|id| graph.edges().iter().any(|edge| touches(edge, id))).expect("a node with an edge");
    let touching: Vec<String> = graph.edges().iter().filter(|edge| touches(edge, &doomed)).map(|edge| edge.id.clone()).collect();
    let apart = graph.edges().into_iter().find(|edge| !touches(edge, &doomed)).map(|edge| edge.id);
    let synapses: Vec<String> = touching.iter().take(1).cloned().chain(apart.clone()).collect();
    let delete = serde_json::json!([{ "operation": "delete", "nodeIds": [doomed, "absent"], "synapseIds": synapses }]).to_string();
    let emit = crate::editor::rewriting::commands::node_graph_edit(&state, TRINITY_REWRITING_PLAY_SURFACE_BEFORE, &delete, "seed").expect("a delete");
    match apart.clone() {
        Some(apart) => assert!(matches!(emit.artifact_mutations.as_slice(), [RewriteRuleMutation::DeleteWorkingNodes(nodes), RewriteRuleMutation::DisconnectWorkingEdges(wires)] if nodes.targets == vec![doomed.clone(), "absent".to_string()] && wires.targets == vec![apart.clone()]), "{:?}", emit.artifact_mutations),
        None => assert!(matches!(emit.artifact_mutations.as_slice(), [RewriteRuleMutation::DeleteWorkingNodes(_)]), "{:?}", emit.artifact_mutations),
    }
    let deleted = applied(&state, &emit.artifact_mutations);
    let after = semio_s_artifact_trinity_jack::JackSnapshot::from_json(&deleted.before_fixture_json).expect("the deleted graph decodes");
    assert!(after.nodes().iter().all(|node| node.id != doomed) && after.nodes().len() + 1 == graph.nodes().len(), "only the named node the graph holds is gone");
    assert_eq!(after.edges().len() + touching.len() + usize::from(apart.is_some()), graph.edges().len(), "every edge touching it and the named wire apart are gone, every other edge stays");
    let outcome = <RewriteRuleMutation as protocol::Mutation<RewritingSnapshot>>::diff(&emit.artifact_mutations[0], &state);
    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.partial"), "the node the graph lacks is skipped: {:?}", outcome.messages());
    let inverse = crate::inverse_rewrite_rule_mutation(&state, &emit.artifact_mutations[0]);
    assert!(matches!(inverse.as_slice(), [RewriteRuleMutation::EditBeforeFixture(_)]), "one inverse row: {inverse:?}");
    let rule = crate::editor::rewriting::commands::node_graph_edit(&state, TRINITY_REWRITING_PLAY_SURFACE_LHS, &serde_json::json!([{ "operation": "delete", "nodeIds": [], "synapseIds": ["lhs-wire"] }]).to_string(), "seed");
    assert!(rule.is_err_and(|fault| fault.code.0 == "trinity.rewriting.node-graph.row"), "a rule wire is cut with its clause, never alone");
}
//#endregion 🕹️NodeDragLaws

```

### 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧪️node-graph-edit-rows/🟦️.ts

SHA-256 beb04a26fda464dfa3b1368df836999e11fa484fd139b0757c68b29384122de9; 4077 bytes.

```typescript
/** 🔗️ Node-graph edit rows (design §13.3): the closed row vocabulary every renderer dispatches as `nodeGraphEdit`
 * arguments, validated by Ajv against the schema, the React journal reader and the flow add-node record — all read from
 * the one language-agnostic fixture the Rust journal encoder and the flow guest decoder check too.
 * @see 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json
 */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020";
import { describe, expect, it } from "vitest";
import { flowAddWidgetArgs, nodeGraphEditRows } from "../../🧱️elements/🕸️NodeGraph/🟦️.tsx";

const engineRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const repoRoot = resolve(engineRoot, "../../../../../..");
const fixture = JSON.parse(readFileSync(resolve(engineRoot, "🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json"), "utf8")) as {
  readonly accepted: readonly { readonly id: string; readonly row: Record<string, unknown> }[];
  readonly refused: readonly { readonly id: string; readonly row: Record<string, unknown> }[];
  readonly answers: readonly { readonly id: string; readonly json: string; readonly rows: readonly Record<string, unknown>[] }[];
  readonly addWidget: readonly { readonly id: string; readonly descriptor: string; readonly x: number; readonly y: number; readonly args: Record<string, unknown> }[];
};
const schema = JSON.parse(readFileSync(resolve(engineRoot, "🧬️schema/🔣️node-graph-edit-rows/🔣️.json"), "utf8"));
const ajv = new Ajv2020({ allErrors: true, strict: true });
ajv.addSchema(schema);
const validateRows = ajv.getSchema(schema.$id)!;
const validateAddWidget = ajv.compile({ $ref: `${schema.$id}#/$defs/AddWidget` });

describe("node-graph edit rows", () => {
  it("accepts every fixture row and refuses every refused one", () => {
    for (const { id, row } of fixture.accepted) expect(validateRows({ operations: [row] }), `${id}: ${JSON.stringify(validateRows.errors)}`).toBe(true);
    for (const { id, row } of fixture.refused) expect(validateRows({ operations: [row] }), id).toBe(false);
  });

  it("reads a host journal answer as its rows and nothing else", () => {
    for (const { id, json, rows } of fixture.answers) expect(nodeGraphEditRows(json), id).toEqual(rows);
    const accepted = fixture.accepted.map(({ row }) => row);
    expect(nodeGraphEditRows(JSON.stringify({ operations: accepted }))).toEqual(accepted);
  });

  it("turns a catalogue descriptor into the addWidget record the wgpu drop dispatches", () => {
    for (const { id, descriptor, x, y, args } of fixture.addWidget) {
      expect(flowAddWidgetArgs(descriptor, x, y), id).toEqual(args);
      expect(validateAddWidget(args), `${id}: ${JSON.stringify(validateAddWidget.errors)}`).toBe(true);
    }
  });

  it("never publishes the whole fixture from either React node-graph surface", () => {
    const reactGraph = readFileSync(resolve(engineRoot, "🧱️elements/🕸️NodeGraph/🟦️.tsx"), "utf8");
    expect(reactGraph).not.toContain("setHostSnapshot");
    expect(reactGraph).not.toContain("commitFixture");
    expect(reactGraph).not.toContain("session.addWidget(");
    const wgpu = readFileSync(resolve(engineRoot, "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs"), "utf8");
    expect(wgpu).not.toContain("setHostSnapshot");
    for (const operation of ["connect", "disconnect", "move", "setSlider", "insertPort"]) expect(wgpu).toContain(`builder.string(Some("operation"), "${operation}")`);
    const dag = readFileSync(resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs"), "utf8");
    for (const operation of ["connect", "disconnect", "move", "setSlider", "insertPort"]) expect(dag).toContain(`row("${operation}", vec![`);
  });
});

```

### 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🧪️node-graph-edit-rows/🦀️.rs

SHA-256 94c37395f42b459688b59a2c9ecf9130e54d2b3e452b1a5d46ab35c9a0c612de; 6163 bytes.

```rust
//! 🔗️ LAW (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING design §13.3): the shared DAG journal encodes every graph edit
//! as exactly the `nodeGraphEdit` row the language-agnostic fixture accepts — the rows React and wgpu both dispatch — and
//! narrates what a gesture or an align moved as one record per distinct offset.
//!
//! Oracle: `📺️renderer/🧑‍🎨engine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json` (schema `🧬️schema/🔣️node-graph-edit-rows`); its
//! TypeScript twin is `📺️renderer/🧑‍🎨engine/🧪️tests/🧪️node-graph-edit-rows/🟦️.ts`.

use super::*;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../../../../../../../📺️renderer/🧑‍🎨engine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json")).expect("node-graph edit rows fixture")
}

/// 🔢️ `value` with every number as `f64`, so an integral JSON number and its float spelling compare equal.
fn normalized(value: &Value) -> Value {
    match value {
        Value::Number(number) => Value::from(number.as_f64().expect("finite number")),
        Value::Array(items) => Value::Array(items.iter().map(normalized).collect()),
        Value::Object(fields) => Value::Object(fields.iter().map(|(key, value)| (key.clone(), normalized(value))).collect()),
        other => other.clone(),
    }
}

/// 🔗️ The journal edit a fixture row narrates; `None` for a row no journal writes (`delete` is a verb's row).
fn journal_edit(row: &Value) -> Option<DagGraphEdit> {
    let text = |field: &str| row[field].as_str().unwrap_or_else(|| panic!("{field}")).to_string();
    Some(match row["operation"].as_str()? {
        "connect" => DagGraphEdit::Connect { source_node_id: text("sourceNodeId"), source_port_id: text("sourcePortId"), target_node_id: text("targetNodeId"), target_port_id: text("targetPortId") },
        "disconnect" => DagGraphEdit::Disconnect { synapse_id: text("synapseId") },
        "move" => DagGraphEdit::Move { gesture_id: text("gestureId"), node_ids: row["nodeIds"].as_array()?.iter().filter_map(|id| id.as_str().map(str::to_string)).collect(), dx: row["dx"].as_f64()?, dy: row["dy"].as_f64()? },
        "setSlider" => DagGraphEdit::SetSlider { node_id: text("widgetId"), value: row["value"].as_f64()? },
        "insertPort" => DagGraphEdit::InsertPort { node_id: text("nodeId"), side: if text("side") == "input" { DagPortSide::Input } else { DagPortSide::Output }, index: usize::try_from(row["index"].as_u64()?).ok()? },
        _ => return None,
    })
}

/// ⚖️ LAW: every journal edit encodes as the very row the fixture accepts, in order, under `operations`.
#[test]
fn every_journal_edit_encodes_as_the_row_the_fixture_accepts() {
    let fixture = fixture();
    let journalled: Vec<Value> = fixture["accepted"].as_array().expect("accepted rows").iter().map(|case| case["row"].clone()).filter(|row| journal_edit(row).is_some()).collect();
    assert_eq!(journalled.len(), 6, "connect, disconnect, move, setSlider and both port sides");
    let edits: Vec<DagGraphEdit> = journalled.iter().filter_map(journal_edit).collect();
    let encoded: Value = serde_json::from_str(&dag_graph_edit_rows_json(edits)).expect("rows json");
    assert_eq!(normalized(&encoded), normalized(&serde_json::json!({ "operations": journalled })));
    assert_eq!(dag_graph_edit_rows_json(Vec::new()), r#"{"operations":[]}"#, "an empty journal is no row");
}

fn node(id: &str, x: f64, y: f64) -> DagNodeSpec {
    let inputs = vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
    let width = computation_node_width(id, &inputs, &outputs);
    let height = computation_node_height(inputs.len(), outputs.len(), false, false);
    DagNodeSpec::computation(id.into(), id, id, "emoji:🔢️".into(), inputs, outputs, false, false, x, y, width, height)
}

fn host(nodes: Vec<DagNodeSpec>) -> DagHost {
    DagHost::from_host_snapshot_without_layout(DagHostSnapshot { schema: "dag.host_snapshot".into(), camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes, edges: vec![] })
}

/// ⚖️ LAW: what moved since a press is one `move` record per distinct offset, the nodes of each in node order; a node that
/// stayed and a node the baseline does not know are not narrated, and a press that moved nothing journals nothing.
#[test]
fn moves_since_a_press_are_one_record_per_distinct_offset() {
    let mut host = host(vec![node("a", 0.0, 0.0), node("b", 100.0, 0.0), node("c", 200.0, 0.0), node("d", 300.0, 0.0)]);
    let baseline = host.node_positions();
    assert!(host.journal_moves_since("node-drag:1", &baseline) && host.take_graph_edits().is_empty(), "nothing moved");
    let moved = [("a", 10.0, 5.0), ("b", 110.0, 5.0), ("c", 200.0, -40.0)];
    for (id, x, y) in moved {
        let node = host.host_snapshot.nodes.iter_mut().find(|node| node.id == id).expect("node");
        (node.x, node.y) = (x, y);
    }
    let without_d: Vec<(String, f64, f64)> = baseline.into_iter().filter(|(id, _, _)| id != "d").collect();
    host.host_snapshot.nodes.iter_mut().find(|node| node.id == "d").expect("d").x = 999.0;
    assert!(host.journal_moves_since("node-drag:2", &without_d));
    assert_eq!(
        host.take_graph_edits(),
        vec![
            DagGraphEdit::Move { gesture_id: "node-drag:2".into(), node_ids: vec!["a".into(), "b".into()], dx: 10.0, dy: 5.0 },
            DagGraphEdit::Move { gesture_id: "node-drag:2".into(), node_ids: vec!["c".into()], dx: 0.0, dy: -40.0 },
        ]
    );
}

/// ⚖️ LAW: a port the embedding host inserted is journalled once, by node, side and index.
#[test]
fn an_inserted_port_is_journalled_once() {
    let mut host = host(vec![node("a", 0.0, 0.0)]);
    assert!(host.journal_port_insert("a".into(), DagPortSide::Input, 2));
    assert!(host.journal_port_insert("a".into(), DagPortSide::Input, 2), "a duplicate is accepted and dropped");
    assert_eq!(host.take_graph_edits(), vec![DagGraphEdit::InsertPort { node_id: "a".into(), side: DagPortSide::Input, index: 2 }]);
}

```

### 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️node-graph-delete-row/🦀️.rs

SHA-256 46d978e2d6c5abbce5aca550fc98dbb424293d09924a13dd465bb1062d2dcf0d; 3024 bytes.

```rust
//! 🗑️ Node-graph context-menu delete laws (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING design §13.3): the delete row
//! names the selection it deletes by id as the ONE `delete {nodeIds, synapseIds}` row of the shared node-graph edit
//! vocabulary — never the ambient `deleteSelection` row the language-agnostic fixture refuses — and an empty selection offers
//! no delete at all.
//! @see 📺️renderer/🧑‍🎨engine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json

use super::*;
use semio_framework_tool_machine::{node_graph_edit_rows, NodeGraphEditRow};

const NODE_GRAPH_EDIT_ROWS_FIXTURE_JSON: &str = include_str!("../../../📺️renderer/🧑‍🎨engine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json");

fn ids(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

/// ⚖️ LAW: via `nodeGraphEdit` the menu row carries the selection's node and edge ids as one shared `delete` row; the direct
/// verb carries no row; an empty selection offers nothing; and the ambient row stays refused by the shared decoder.
#[test]
fn a_menu_delete_names_its_selection_as_one_delete_row() {
    let spec = node_graph_delete_selection_spec("Delete selection", false, &ids(&["a", "b"]), &ids(&["s1"]), NodeGraphDeleteDispatch::ViaNodeGraphEdit).expect("a selection offers delete");
    assert_eq!((spec.id.as_str(), spec.action.as_deref(), spec.destructive), ("delete-selection", Some("nodeGraphEdit"), Some(true)));
    let args = spec.args.expect("the delete row");
    assert_eq!(node_graph_edit_rows(&args), Ok(vec![NodeGraphEditRow::Delete { node_ids: ids(&["a", "b"]), synapse_ids: ids(&["s1"]) }]));
    let edges_only = node_graph_delete_selection_spec("Delete selection", true, &[], &ids(&["s1", "s2"]), NodeGraphDeleteDispatch::ViaNodeGraphEdit).expect("edges alone offer delete");
    assert_eq!(node_graph_edit_rows(&edges_only.args.expect("the delete row")), Ok(vec![NodeGraphEditRow::Delete { node_ids: Vec::new(), synapse_ids: ids(&["s1", "s2"]) }]));
    let direct = node_graph_delete_selection_spec("Delete selection", false, &ids(&["a"]), &[], NodeGraphDeleteDispatch::Direct).expect("a selection offers delete");
    assert_eq!((direct.action.as_deref(), direct.args.is_none()), (Some("deleteSelection"), true));
    for dispatch in [NodeGraphDeleteDispatch::ViaNodeGraphEdit, NodeGraphDeleteDispatch::Direct] {
        assert!(node_graph_delete_selection_spec("Delete selection", false, &[], &[], dispatch).is_none(), "an empty selection offers no delete");
    }
    let fixture: serde_json::Value = serde_json::from_str(NODE_GRAPH_EDIT_ROWS_FIXTURE_JSON).expect("node-graph edit rows fixture");
    let ambient = fixture["refused"].as_array().expect("refused rows").iter().find(|case| case["id"] == "ambient-selection-delete").expect("the ambient delete row");
    assert!(node_graph_edit_rows(&DslValue::from(&serde_json::json!({ "operations": [ambient["row"].clone()] }))).is_err(), "the ambient row is refused");
}

```

### 🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🔬️node-graph-edit-rows/🦀️.rs

SHA-256 e3753f6c2907e6318ff86b0454e64acfd0e33e3a9c7eba930eed6bd09a0609c1; 5134 bytes.

```rust
//! 🔬️ The shared `nodeGraphEdit` row decoder against the renderer's committed row vocabulary fixture: every accepted row
//! decodes to its typed record, every refused row (whole fixtures, ambient selections, absolute moves, extra fields, empty
//! or repeated ids, text numbers, unknown sides, non-integer indices, empty deletes, unknown operations) is refused.

use super::*;
use serde_json::Value;

const ROWS: &str = include_str!("../../../../🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json");

fn dsl(value: &Value) -> protocol::DslValue {
    match value {
        Value::Null => protocol::DslValue::Null,
        Value::Bool(flag) => protocol::DslValue::Bool(*flag),
        Value::Number(number) => protocol::DslValue::float(number.as_f64().expect("fixture numbers are finite")),
        Value::String(text) => protocol::DslValue::String(text.clone()),
        Value::Array(items) => protocol::DslValue::Array(items.iter().map(dsl).collect()),
        Value::Object(entries) => protocol::DslValue::object(entries.iter().map(|(key, value)| (key.clone(), dsl(value)))),
    }
}

fn cases(group: &str) -> Vec<(String, protocol::DslValue)> {
    let fixture: Value = serde_json::from_str(ROWS).expect("the row fixture parses");
    fixture[group].as_array().expect("a case group").iter().map(|case| (case["id"].as_str().expect("case id").to_string(), dsl(&case["row"]))).collect()
}

/// ✅️ Every accepted row decodes, to exactly the record the row names.
#[test]
fn every_accepted_row_decodes_to_its_record() {
    let decoded: Vec<(String, NodeGraphEditRow)> = cases("accepted").into_iter().map(|(id, row)| (id.clone(), NodeGraphEditRow::from_row(&row).unwrap_or_else(|reason| panic!("accepted row {id} refused: {reason}")))).collect();
    let by_id = |id: &str| decoded.iter().find(|(case, _)| case == id).map(|(_, row)| row.clone()).expect("fixture case");
    assert_eq!(by_id("connect"), NodeGraphEditRow::Connect { source_node_id: "slider".into(), source_port_id: "number".into(), target_node_id: "add".into(), target_port_id: "b".into() });
    assert_eq!(by_id("disconnect"), NodeGraphEditRow::Disconnect { synapse_id: "s1".into() });
    assert_eq!(by_id("move"), NodeGraphEditRow::Move(NodeDragRecord { gesture_id: "node-drag:3".into(), node_ids: vec!["add".into(), "slider".into()], dx: 284.5, dy: -48.0 }));
    assert_eq!(by_id("set-slider"), NodeGraphEditRow::SetSlider { widget_id: "slider".into(), value: 6.5 });
    assert_eq!(by_id("insert-input-port"), NodeGraphEditRow::InsertPort { node_id: "add".into(), side: NodePortSide::Input, index: 2 });
    assert_eq!(by_id("insert-output-port"), NodeGraphEditRow::InsertPort { node_id: "add".into(), side: NodePortSide::Output, index: 0 });
    assert_eq!(by_id("delete-nodes-and-wires"), NodeGraphEditRow::Delete { node_ids: vec!["add".into()], synapse_ids: vec!["s1".into()] });
    assert_eq!(by_id("delete-wires-only"), NodeGraphEditRow::Delete { node_ids: Vec::new(), synapse_ids: vec!["s1".into(), "s2".into()] });
}

/// 🚫️ Every refused row is refused, and so is any batch that carries it.
#[test]
fn every_refused_row_is_refused_alone_and_in_a_batch() {
    for (id, row) in cases("refused") {
        assert!(NodeGraphEditRow::from_row(&row).is_err(), "refused row {id} decoded");
        let accepted = cases("accepted").into_iter().map(|(_, row)| row);
        let batch = protocol::DslValue::object([("operations".to_string(), protocol::DslValue::Array(accepted.chain([row]).collect()))]);
        assert!(node_graph_edit_rows(&batch).is_err(), "a batch carrying refused row {id} decoded");
    }
}

/// 🧾️ The arguments are closed: the rows plus the scrub press fields decode (an empty abort batch too); a foreign root
/// field, a missing `operations` or more than [`NODE_GRAPH_EDIT_MAX_ROWS`] rows refuse the batch.
#[test]
fn the_arguments_are_a_closed_bounded_batch() {
    let rows: Vec<protocol::DslValue> = cases("accepted").into_iter().map(|(_, row)| row).collect();
    let args = |fields: Vec<(&str, protocol::DslValue)>| protocol::DslValue::object(fields.into_iter().map(|(key, value)| (key.to_string(), value)));
    assert_eq!(node_graph_edit_rows(&args(vec![("operations", protocol::DslValue::Array(rows.clone()))])).expect("accepted batch").len(), rows.len());
    let abort = args(vec![("operations", protocol::DslValue::Array(Vec::new())), (SCRUB_GESTURE_ARG, protocol::DslValue::String("slider:7".into())), (SCRUB_ABORT_ARG, protocol::DslValue::String("blur".into()))]);
    assert_eq!(node_graph_edit_rows(&abort).expect("an abort batch"), Vec::new());
    assert!(node_graph_edit_rows(&args(vec![("operations", protocol::DslValue::Array(rows.clone())), ("hostSnapshotChanged", protocol::DslValue::Bool(true))])).is_err());
    assert!(node_graph_edit_rows(&args(vec![(SCRUB_GESTURE_ARG, protocol::DslValue::String("slider:7".into()))])).is_err());
    let flood = vec![rows[1].clone(); NODE_GRAPH_EDIT_MAX_ROWS + 1];
    assert!(node_graph_edit_rows(&args(vec![("operations", protocol::DslValue::Array(flood))])).is_err());
}

```

### ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs

SHA-256 b4d7dbdb3e7cad6209daee67902803ff8828f8386a20383558019851ff68eb26; 35894 bytes.

```rust
pub(crate) mod context {
    use super::super::*;
    use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry_and_members};
    use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};
    
    /// ✏️ `EquationPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
    /// `ArtifactApp` — `EditorApp<EquationPlayApp>` (SDK adapter, contract §2.1) is the real
    /// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
    /// `PluginBuilder::editor::<EquationPlayApp>` builds it.
    pub type MathApp = VcsArtifactApp<EditorApp<EquationPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>;
    
    /// 🧪️ The app instance every test builds — registry-backed, because there is no other kind.
    /// `EditorApp<EquationPlayApp>` publishes a `bounded_first_step_tool_proofs!` roster, and
    /// `with_registry_on_bus` joins that roster against the registry's `Migrated` tool ids
    /// (`AppActionRegistry::validate_tool_job_rows`): an empty registry declares none of them, so the
    /// registry-LESS `artifact_app_laws::new_app` fails construction outright with
    /// `interactive-job.catalog-authority … generated_migrated=false, migrated={}`.
    pub async fn math_app() -> OwnedMathApp {
        math_app_with_registry().await
    }
    
    /// ✏️ Adapts `create_equation_app`'s `AppDefinition` (contract §2.4) into the `App {
    /// definition, examples }` shape `context::assert_declared_actions_bridge_to_commands` still
    /// expects — framework test context gap, not modifiable here (`🧰️framework/**` is outside this
    /// packet's lease).
    pub fn equation_app_manifest_for_tests() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_equation_app(), examples: Vec::new() }
    }

    semio_framework_plugin::history_edit_acceptance_law!("mathematical", EquationPlayApp, equation_app_manifest_for_tests, "../..");
    
    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    /// 🪪️ Bound to the live runtime instance `meta("local")` addresses. Binding is mandatory now that
    /// every document verb is classified `Migrated`: an unbound wrapper answers every typed dispatch
    /// `interactive-job.live-instance: typed command … does not belong to the mounted live app instance`.
    pub async fn math_app_with_registry() -> OwnedMathApp {
        let mut app = new_app_with_registry_and_members::<EditorApp<EquationPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(equation_app_manifest_for_tests).await;
        app.bind_instance_id(meta("local").instance_id).await;
        OwnedMathApp(app)
    }

    /// 🔁️ Drives one dispatched typed operation to quiescence the way the plugin host does — on a
    /// mounted app `dispatch_typed` only QUEUES the operation, so a test reading `app.snapshot()`
    /// straight afterwards would observe the pre-dispatch document.
    pub async fn settle(app: &mut MathApp) {
        semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, meta("local").instance_id).await.expect("settle the typed operation");
    }

    /// 🔚 A mounted app that RETIRES ITSELF. A live `ArtifactStore` asserts in `Drop`
    /// (`artifact store reached Drop without its exact terminal-empty shallow-shell witness`) unless
    /// it walked its bounded close loop first, so the fixture owns the close instead of asking every
    /// law to remember a trailing `close(&mut app)` — which is what makes a law that fails an
    /// assertion report ITS failure instead of a close panic. Skipped while unwinding, where the
    /// original panic is the report worth keeping. Derefs to the bare app for every read and dispatch.
    pub struct OwnedMathApp(MathApp);

    impl OwnedMathApp {
        /// 🔚 Walks the bounded close protocol; idempotent (a terminal-empty app returns at once).
        pub fn close(&mut self) {
            semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0);
        }
    }

    impl std::ops::Deref for OwnedMathApp {
        type Target = MathApp;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl std::ops::DerefMut for OwnedMathApp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Drop for OwnedMathApp {
        fn drop(&mut self) {
            if !std::thread::panicking() {
                self.close();
            }
        }
    }
    
    pub async fn dispatch(app: &mut MathApp, command: EquationCommand) -> InvocationResult {
        let result = app.dispatch_typed(command, &meta("local")).await.expect("dispatch");
        settle(app).await;
        result
    }
    
    pub async fn render(app: &mut MathApp, body_key: &str) -> String {
        // 🌱️ `UiNode` (`semio-framework-plugin`, framework-owned) has not itself gained `ToValue` —
        // `Debug` gives every test caller here the same "does the render mention X" substring check.
        format!("{:?}", app.render(body_key, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render"))
    }
}

use super::*;
use crate::editor::equation::unit_tests::context::{math_app, math_app_with_registry};

//#region 🔖️RetainedCommands
fn retained_operation(generation: u64) -> AppOperationContext {
    AppOperationContext { app_instance_id: 7, parent_document_id: "equation-retained-test".into(), operation_id: 11, generation, canonical_base_revision: [17; 32], authoring_seed: "authoring-seed-test".into() }
}

fn graph_with_shape(node_count: usize, edge_count: usize) -> EquationGraph {
    let nodes = (0..node_count).map(|index| crate::EquationNode { id: format!("n{index}"), label: format!("N{index}"), x: index as f64, y: -(index as f64) }).collect();
    let edges = (0..edge_count).map(|index| crate::EquationEdge { id: format!("e{index}"), source: format!("n{}", index % node_count.max(1)), target: format!("n{}", (index + 1) % node_count.max(1)) }).collect();
    EquationGraph { directed: true, nodes, edges, algorithm: "bfs".into(), algorithm_seed: Some("n0".into()) }
}

fn drive_retained(work: &mut EquationRetainedCommandWork, command: &EquationCommand, snapshot: &EquationSnapshot, operation: &AppOperationContext) -> protocol::DslValue {
    let config = NoConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    loop {
        match work.step(&semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation }).expect("retained Equation turn") {
            ArtifactCommandWorkStep::Replay { .. } | ArtifactCommandWorkStep::Progress { .. } => {}
            // 🌱️ `ToValue`/`DslValue` in place of the old `serde_json::to_value` oracle: `DslValue`
            // already implements `PartialEq`, so the two runs compare directly with no JSON text
            // round trip needed.
            ArtifactCommandWorkStep::Complete(emit) => return protocol::ToValue::to_value(&emit.artifact_mutations),
            ArtifactCommandWorkStep::CompleteWithEphemeral { .. } => panic!("Equation commands do not publish ephemeral state"),
        }
    }
}

#[test]
fn retained_schema_contract_and_factory_identity_are_exact() {
    let fixture: Value = json::parse(include_str!("../../../../../../../🧫️fixtures/⚖️equation-retained-command-law.json")).expect("language-neutral retained fixture");
    assert_eq!(fixture["contract"]["workItems"], 65_536);
    assert_eq!(fixture["contract"]["maximumStepMillis"], 8);
    assert_eq!(fixture["actions"], json::array(EQUATION_TOOL_IDS.iter().map(|id| Value::from(*id))));
    assert_eq!(fixture["hostileCases"].as_array().map(|values| values.len()), Some(14));
    let factory = EquationCommandJobFactory::new("s.mathematical.equation@1/*#editor");
    let keys = <EquationCommandJobFactory as semio_framework::ToolJobFactory>::keys(&factory);
    assert_eq!(keys.len(), EQUATION_TOOL_IDS.len());
    for (key, tool_id) in keys.iter().zip(EQUATION_TOOL_IDS) {
        assert_eq!(key.controller_id, "s.mathematical.equation@1/*#editor");
        assert_eq!(key.tool_id, *tool_id);
    }
    assert_eq!(<EquationPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), 8);
}

#[semio_framework_async_macros::async_test]
async fn retained_semantic_maxima_accept_exact_and_reject_maximum_plus_one() {
    let command = EquationCommand::SetDirected(set_directed::SetDirected { directed: false });
    let maximum_nodes = crate::equation_snapshot_with_state(&graph_with_shape(EQUATION_MAX_NODES, 0), &EquationGeometry::default());
    let excessive_nodes = crate::equation_snapshot_with_state(&graph_with_shape(EQUATION_MAX_NODES + 1, 0), &EquationGeometry::default());
    assert!(equation_command_extent(&command, &maximum_nodes).is_some());
    assert!(equation_command_extent(&command, &excessive_nodes).is_none());
    let maximum_edges = crate::equation_snapshot_with_state(&graph_with_shape(2, EQUATION_MAX_EDGES), &EquationGeometry::default());
    let excessive_edges = crate::equation_snapshot_with_state(&graph_with_shape(2, EQUATION_MAX_EDGES + 1), &EquationGeometry::default());
    assert!(equation_command_extent(&command, &maximum_edges).is_some());
    assert!(equation_command_extent(&command, &excessive_edges).is_none());

    let snapshot = crate::equation_snapshot_with_state(&EquationGraph::default(), &EquationGeometry::default());
    let point = crate::EquationPoint { x: 1.0, y: 2.0 };
    let maximum_points = EquationCommand::SetPoints(set_points::SetPoints { geometry: EquationGeometry { points: vec![point.clone(); EQUATION_MAX_POINTS] } });
    let excessive_points = EquationCommand::SetPoints(set_points::SetPoints { geometry: EquationGeometry { points: vec![point; EQUATION_MAX_POINTS + 1] } });
    assert!(equation_command_extent(&maximum_points, &snapshot).is_some());
    assert!(equation_command_extent(&excessive_points, &snapshot).is_none());
    let maximum_text = "a".repeat(EQUATION_MAX_TEXT_BYTES);
    let excessive_text = "a".repeat(EQUATION_MAX_TEXT_BYTES + 1);
    assert!(equation_command_extent(&EquationCommand::SetAlgorithm(set_algorithm::SetAlgorithm { algorithm: maximum_text, seed: None }), &snapshot).is_some());
    assert!(equation_command_extent(&EquationCommand::SetAlgorithm(set_algorithm::SetAlgorithm { algorithm: excessive_text, seed: None }), &snapshot).is_none());

    let operations = |count: usize| json::to_string(&json::array(std::iter::repeat(json::object([("operation".to_string(), Value::from("disconnect")), ("synapseId".to_string(), Value::from("e1"))])).take(count)));
    assert!(equation_command_extent(&EquationCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: operations(EQUATION_MAX_EDIT_OPERATIONS) }), &snapshot).is_some());
    assert!(equation_command_extent(&EquationCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: operations(EQUATION_MAX_EDIT_OPERATIONS + 1) }), &snapshot).is_none());
    let delete = |count: usize| json::to_string(&json::array([json::object([("operation".to_string(), Value::from("delete")), ("nodeIds".to_string(), json::array((0..count).map(|index| Value::from(format!("n{index}"))))), ("synapseIds".to_string(), json::array([]))])]));
    assert!(equation_command_extent(&EquationCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: delete(EQUATION_MAX_DELETE_IDS) }), &snapshot).is_some());
    assert!(equation_command_extent(&EquationCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: delete(EQUATION_MAX_DELETE_IDS + 1) }), &snapshot).is_none());
    let exact_json = format!("[{}]", " ".repeat(EQUATION_MAX_EDIT_JSON_BYTES - 2));
    let excessive_json = format!("[{}]", " ".repeat(EQUATION_MAX_EDIT_JSON_BYTES - 1));
    assert_eq!(exact_json.len(), EQUATION_MAX_EDIT_JSON_BYTES);
    assert_eq!(excessive_json.len(), EQUATION_MAX_EDIT_JSON_BYTES + 1);
    assert!(equation_command_extent(&EquationCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: exact_json }), &snapshot).is_some());
    assert!(equation_command_extent(&EquationCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: excessive_json }), &snapshot).is_none());
}

#[semio_framework_async_macros::async_test]
async fn retained_interruption_replay_aba_cancel_and_repeated_close_are_exact() {
    let graph = graph_with_shape(8, 12);
    let snapshot = crate::equation_snapshot_with_state(&graph, &EquationGeometry::default());
    let command = EquationCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit {
        operations_json: json::to_string(&json::array([
            json::object([("operation".to_string(), Value::from("move")), ("gestureId".to_string(), Value::from("node-drag:7")), ("nodeIds".to_string(), json::array([Value::from("n7")])), ("dx".to_string(), Value::from(41.0)), ("dy".to_string(), Value::from(42.0))]),
            json::object([("operation".to_string(), Value::from("delete")), ("nodeIds".to_string(), json::array([Value::from("n1"), Value::from("n3")])), ("synapseIds".to_string(), json::array([]))]),
            json::object([("operation".to_string(), Value::from("connect")), ("sourceNodeId".to_string(), Value::from("n2")), ("sourcePortId".to_string(), Value::from("")), ("targetNodeId".to_string(), Value::from("n5")), ("targetPortId".to_string(), Value::from(""))]),
        ])),
    });
    let operation = retained_operation(13);
    let extent = equation_command_extent(&command, &snapshot).expect("retained extent");
    let identity = equation_operation_identity("nodeGraphEdit", &operation);
    let mut uninterrupted = EquationRetainedCommandWork::new("nodeGraphEdit", identity, extent);
    let config = NoConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    for _ in 0..9 {
        assert!(matches!(
            uninterrupted
                .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs { command: &command, snapshot: &snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation })
                .expect("checkpoint prefix"),
            ArtifactCommandWorkStep::Progress { .. }
        ));
    }
    let mut checkpoint = [0_u8; 40];
    assert_eq!(uninterrupted.checkpoint(&mut checkpoint).expect("checkpoint"), 40);
    let aba_operation = retained_operation(14);
    let mut stale_aba = EquationRetainedCommandWork::new("nodeGraphEdit", equation_operation_identity("nodeGraphEdit", &aba_operation), extent);
    assert!(stale_aba.restore(&checkpoint).is_err());
    let mut wrong_action = EquationRetainedCommandWork::new("setDirected", equation_operation_identity("setDirected", &operation), extent);
    assert!(wrong_action.restore(&checkpoint).is_err());

    let mut replayed = EquationRetainedCommandWork::new("nodeGraphEdit", identity, extent);
    replayed.restore(&checkpoint).expect("interrupted restore");
    let uninterrupted_output = drive_retained(&mut uninterrupted, &command, &snapshot, &operation);
    let replayed_output = drive_retained(&mut replayed, &command, &snapshot, &operation);
    assert_eq!(uninterrupted_output, replayed_output, "the DslValue-encoded mutation output must observe exact replay output");

    let mut cancelled_before = EquationRetainedCommandWork::new("nodeGraphEdit", identity, extent);
    assert_eq!(cancelled_before.close_step(1, usize::MAX), InteractiveJobCloseStep::Blocked);
    cancelled_before.begin_close();
    assert_eq!(cancelled_before.close_step(1, usize::MAX), InteractiveJobCloseStep::Complete);
    assert_eq!(cancelled_before.close_step(1, usize::MAX), InteractiveJobCloseStep::Complete);
    let mut cancelled_after = EquationRetainedCommandWork::new("nodeGraphEdit", identity, extent);
    assert!(matches!(
        cancelled_after
            .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs { command: &command, snapshot: &snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation })
            .expect("cancel after admission"),
        ArtifactCommandWorkStep::Progress { .. }
    ));
    cancelled_after.begin_close();
    assert!(matches!(cancelled_after.close_step(0, 0), InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 }));
    while !cancelled_after.terminal_is_empty() {
        let _ = cancelled_after.close_step(1, usize::MAX);
    }
    assert_eq!(cancelled_after.close_step(1, usize::MAX), InteractiveJobCloseStep::Complete);
    assert_eq!(cancelled_after.close_step(1, usize::MAX), InteractiveJobCloseStep::Complete);
}

/// ⏱️ The runtime's own step law over the maximum document: `StepOverrunLedger` records any turn past
/// `INTERACTIVE_STEP_CEILING_US` and quarantines a session only after
/// `SUSTAINED_OVERRUN_QUARANTINE_STEPS` consecutive overruns. So one descheduled turn on a loaded host is
/// forgiven, and a turn whose own cost exceeds the ceiling overruns every time and is convicted. A turn
/// that re-derived the whole document (the per-step extent re-check) failed this law on every run.
#[semio_framework_async_macros::async_test]
async fn retained_maximum_microturns_stay_below_eight_milliseconds() {
    let ceiling = std::time::Duration::from_micros(semio_framework_job::INTERACTIVE_STEP_CEILING_US);
    let mut consecutive = 0_u32;
    let mut admit = |elapsed: std::time::Duration, turn: &str| {
        consecutive = if elapsed < ceiling { 0 } else { consecutive + 1 };
        assert!(consecutive < semio_framework_job::SUSTAINED_OVERRUN_QUARANTINE_STEPS, "maximum Equation {turn} turns overran {ceiling:?} {consecutive} times in a row (last {elapsed:?})");
    };
    let graph = graph_with_shape(EQUATION_MAX_NODES, EQUATION_MAX_EDGES);
    let snapshot = crate::equation_snapshot_with_state(&graph, &EquationGeometry::default());
    let ids = (0..EQUATION_MAX_DELETE_IDS).map(|index| format!("n{index}")).collect::<Vec<_>>();
    let mut operations = vec![json::object([("operation".to_string(), Value::from("delete")), ("nodeIds".to_string(), json::array(ids.iter().map(|id| Value::from(id.as_str())))), ("synapseIds".to_string(), json::array([]))])];
    operations.resize(EQUATION_MAX_EDIT_OPERATIONS, json::object([("operation".to_string(), Value::from("disconnect")), ("synapseId".to_string(), Value::from("e1"))]));
    let command = EquationCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: json::to_string(&json::array(operations)) });
    let operation = retained_operation(23);
    let extent = equation_command_extent(&command, &snapshot).expect("maximum retained extent");
    let mut work = EquationRetainedCommandWork::new("nodeGraphEdit", equation_operation_identity("nodeGraphEdit", &operation), extent);
    let config = NoConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    loop {
        let started = std::time::Instant::now();
        let step = work
            .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs { command: &command, snapshot: &snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation })
            .expect("maximum retained turn");
        admit(started.elapsed(), "micro");
        if matches!(step, ArtifactCommandWorkStep::Complete(_)) {
            break;
        }
    }
    work.begin_close();
    while !work.terminal_is_empty() {
        let started = std::time::Instant::now();
        let _ = work.close_step(1, usize::MAX);
        admit(started.elapsed(), "close");
    }
}
//#endregion 🔖️RetainedCommands

//#region 🔖️NodeGraphRows
/// 🧾️ The renderer's committed node-graph row vocabulary (schema `📺️renderer/🧑‍🎨engine/🧬️schema/🔣️node-graph-edit-rows`).
const NODE_GRAPH_EDIT_ROWS: &str = include_str!("../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json");

/// ⚖️ LAW: the equation guest decodes the renderer's committed node-graph rows through the ONE shared decoder: every refused
/// row is refused, every accepted row its graph carries (`move`, `connect`, `disconnect`, `delete`) decodes, and the
/// `setSlider`/`insertPort` rows it has no widget for are refused by name.
#[test]
fn the_renderer_row_fixture_decodes_exactly() {
    let fixture: Value = json::parse(NODE_GRAPH_EDIT_ROWS).expect("the row fixture parses");
    for case in fixture["accepted"].as_array().expect("accepted rows") {
        let carried = !matches!(case["row"]["operation"].as_str(), Some("setSlider" | "insertPort"));
        assert_eq!(EquationEditOperation::from_value(&case["row"]).is_ok(), carried, "accepted row {:?}", case["id"].as_str());
    }
    for case in fixture["refused"].as_array().expect("refused rows") {
        assert!(EquationEditOperation::from_value(&case["row"]).is_err(), "refused row {:?} decoded", case["id"].as_str());
    }
}
//#endregion 🔖️NodeGraphRows

//#region 🔖️CommandSurface
/// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
/// wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique_and_the_full_row_set_is_covered() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), 8, "every EquationCommand row must be covered by every_command()");
}

/// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — the
/// (an undeclared host-pushed command) and `setDocument` → `set-artifact` (the `app_commands!`
/// row's own `"setDocument" as "set-artifact" => set_artifact::SetArtifact` explicitly pins a
/// non-kebab wire keyword, matching `SetArtifact`'s own `#[dsl(keyword = "set-artifact")]`).
/// **Pre-existing bug, independently traced**: `git log -1 --date=iso -- 🎮️commands/🗿️set-artifact/
/// 🦀️.rs` shows `SetArtifact`'s explicit `set-artifact` keyword predates this ticket's
/// own edits to this file (which only touched `render`/`export_media`); this test's hardcoded
/// exception list simply never accounted for the second declared divergence. Fixed outright
/// per this ticket's own "trivial, safe, unambiguous" guidance rather than left unresolved.
#[semio_framework_async_macros::async_test]
async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
    for command in every_command() {
        let id = command.command_id();
        let expected = match id {
            "setDocument" => "set-artifact".to_string(),
            _ => id.chars().flat_map(|c| if c.is_ascii_uppercase() { vec!['-', c.to_ascii_lowercase()] } else { vec![c] }).collect(),
        };
        let printed = protocol::OpText::print_op(&command);
        assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {id}: {printed:?}");
    }
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(super) fn every_command() -> Vec<EquationCommand> {
    vec![
        EquationCommand::SetArtifact(set_artifact::SetArtifact { graph: crate::document_dsl::math_graph_to_dsl(&EquationGraph::default()), geometry: EquationGeometry::default() }),
        EquationCommand::SetAlgorithm(set_algorithm::SetAlgorithm { algorithm: "bfs".into(), seed: Some("a".into()) }),
        EquationCommand::SetDirected(set_directed::SetDirected { directed: true }),
        EquationCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: r#"[{"operation":"move","gestureId":"node-drag:1","nodeIds":["a"],"dx":12.0,"dy":34.0}]"#.into() }),
        EquationCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: semio_framework_os_kernel::Viewport2d { x: 5.0, y: 6.0, zoom: 2.0 } }),
        EquationCommand::SetPoints(set_points::SetPoints { geometry: EquationGeometry::default() }),
        EquationCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::examples::demo::ID.into() }),
        EquationCommand::AddNode(crate::editor::equation::commands::add_node::AddNode { x: 12.0, y: 34.0 }),
    ]
}

/// ⚖️ The row whose `Option` field makes `None`/`Some` distinct wire cases, pinned to the exact bytes
/// captured from the pre-merge `equation_protocol` crate (see the ticket's
/// `🧪️wire-baseline-before.txt`).
#[semio_framework_async_macros::async_test]
async fn optional_field_rows_keep_their_pre_migration_bytes() {
    let cases: [(EquationCommand, &str, &str); 2] = [
        (EquationCommand::SetAlgorithm(set_algorithm::SetAlgorithm { algorithm: "topo".into(), seed: None }), "set-algorithm algorithm=topo", "01010104746f706f01000600"),
        (EquationCommand::SetAlgorithm(set_algorithm::SetAlgorithm { algorithm: "bfs".into(), seed: Some("a".into()) }), "set-algorithm algorithm=bfs seed=a", "01010201610362667302000601010600"),
    ];
    for (command, text, hex) in cases {
        assert_eq!(protocol::OpText::print_op(&command), text);
        assert_eq!(protocol::OpBinary::encode_op(&command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), hex);
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}
//#endregion 🔖️CommandSurface

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    // 🌱️ `AppDefinition` (`semio-framework-plugin`, framework-owned) has not itself gained
    // `ToValue` — `Debug` gives the same "does the manifest mention X" substring check without
    // needing `serde_json` for a framework type this batch does not own.
    let debug = format!("{:?}", create_equation_app());
    for id in [graph_window::MATH_PLAY_WINDOW_GRAPH, geometry_window::MATH_PLAY_WINDOW_GEOMETRY] {
        assert!(debug.contains(id), "window kind {id} missing from the manifest: {debug}");
    }
    assert!(debug.contains(edit::MATH_PLAY_MODE_EDIT), "mode missing from the manifest");
    assert!(debug.contains("computation.equation"), "artifact kind missing from the manifest");
}

#[semio_framework_async_macros::async_test]
async fn equation_io_is_declared_on_the_manifest() {
    let app = create_equation_app();
    assert_eq!(app.io.artifact.id, "computation.equation");
    assert_eq!(app.io.ports.len(), 1);
    assert_eq!(app.io.ports[0].id, "result:out");
}

#[semio_framework_async_macros::async_test]
async fn create_equation_app_builds_a_definition_for_the_editor_role() {
    let def = create_equation_app();
    assert_eq!(def.role, semio_framework::AppRole::Editor);
    assert_eq!(def.dialect, EQUATION_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<EquationPlayApp as ArtifactEditor>::DIALECT, EQUATION_DIALECT);
}
//#endregion 🔖️ManifestSanity

//#region 🔖️CrossCutting
#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    use crate::editor::equation::unit_tests::context::render;
    let mut app = math_app().await;
    assert!(render(&mut app, "equation.play.nope").await.contains("Unknown body"));
}

#[semio_framework_async_macros::async_test]
async fn command_surface_is_registry_clean() {
    let _app = math_app_with_registry().await;
}
//#endregion 🔖️CrossCutting

//#region 🔖️EquationIo
#[semio_framework_async_macros::async_test]
async fn equation_io_declares_result_out_with_the_computation_equation_kind() {
    let io = equation_io();
    assert_eq!(io.artifact_schema, "semio.equation/v1");
    assert_eq!(io.artifact.id, "computation.equation");
    assert_eq!(io.ports.len(), 1);
    let port = &io.ports[0];
    assert_eq!(port.id, "result:out");
    assert_eq!(port.kind_id.as_deref(), Some("computation.equation"));
    assert_eq!(port.direction, semio_framework_plugin::MediaPortDirection::Out);
    assert_eq!(port.multiplicity, semio_framework::PortMultiplicity::Many);
    assert!(!port.required);
}
//#endregion 🔖️EquationIo

//#region 🔖️GraphAlgorithms
#[semio_framework_async_macros::async_test]
async fn topo_algorithm_overlay_orders_dag_nodes() {
    let graph = EquationGraph::default();
    let overlay = algorithm_overlay(&graph);
    assert!(overlay.get("a").unwrap().starts_with(" #0"));
    assert!(overlay.get("d").unwrap().starts_with(" #"));
}

#[semio_framework_async_macros::async_test]
async fn components_algorithm_overlay_groups_disconnected_node() {
    use crate::EquationNode;
    let mut graph = EquationGraph { algorithm: "components".into(), ..EquationGraph::default() };
    graph.nodes.push(EquationNode { id: "z".into(), label: "Z".into(), x: 0.0, y: 0.0 });
    let overlay = algorithm_overlay(&graph);
    assert_ne!(overlay.get("a"), overlay.get("z"));
}

#[semio_framework_async_macros::async_test]
async fn bfs_algorithm_overlay_reports_hop_distance() {
    let graph = EquationGraph { algorithm: "bfs".into(), algorithm_seed: Some("a".into()), ..EquationGraph::default() };
    let overlay = algorithm_overlay(&graph);
    assert_eq!(overlay.get("a").unwrap(), " d0");
    assert_eq!(overlay.get("b").unwrap(), " d1");
}

#[semio_framework_async_macros::async_test]
async fn workflow_json_round_trips_node_count() {
    let graph = EquationGraph::default();
    let (nodes, edges) = workflow_json(&graph);
    assert_eq!(nodes.len(), graph.nodes.len());
    assert_eq!(edges.len(), graph.edges.len());
}

/// 🔌️ Every edge endpoint names a port its node declares — the node-graph engine resolves an edge only
/// through a declared port, so port-less nodes drew no edge at all.
#[semio_framework_async_macros::async_test]
async fn workflow_json_edges_end_on_declared_ports() {
    let (nodes, edges) = workflow_json(&EquationGraph::default());
    assert!(!edges.is_empty());
    for edge in &edges {
        let source = nodes.iter().find(|node| node.id == edge.source_node_id).expect("source node");
        let target = nodes.iter().find(|node| node.id == edge.target_node_id).expect("target node");
        assert!(source.outputs.iter().any(|port| port.id == edge.source_port_id), "{} leaves an undeclared port", edge.id);
        assert!(target.inputs.iter().any(|port| port.id == edge.target_port_id), "{} enters an undeclared port", edge.id);
    }
}
//#endregion 🔖️GraphAlgorithms

//#region 🔖️Geometry
#[semio_framework_async_macros::async_test]
async fn geometry_layers_include_hull_and_centroid() {
    let geometry = EquationGeometry::default();
    let layers_json = geometry_layers_json(&geometry);
    assert!(layers_json.contains("\"hull\""));
    assert!(layers_json.contains("\"centroid\""));
}
//#endregion 🔖️Geometry

//#region 🔖️LoadedDocumentDispatch
/// 🧩️ A document that reaches the app through the VALUE projection carries no local
/// `EquationWorkingScene` owner — `to_value` writes the three child handles only (the scene owner law's
/// `wireOmission` case). Every document verb used to be measured against that owner and refused with
/// `equation-command-capacity` (measured live 2026-09-20, slice PB2). The extent reads the fail-soft
/// projection, so an owner-less snapshot stays editable. The text and pack codecs carry the scene since
/// ticket 26/09/19 and are no longer an owner-less transport.
#[semio_framework_async_macros::async_test]
async fn a_decoded_document_without_a_scene_owner_still_admits_its_document_verbs() {
    let authored = crate::equation_snapshot_with_state(&graph_with_shape(4, 3), &EquationGeometry::default());
    assert!(crate::equation_scene_owner(&authored).is_some(), "an authored snapshot mints the live scene owner");
    let decoded = <crate::EquationSnapshot as semio_framework_os_kernel::FromValue>::from_value(semio_framework_os_kernel::ToValue::to_value(&authored)).expect("value projection decodes");
    assert!(crate::equation_scene_owner(&decoded).is_none(), "a value-decoded snapshot carries no local owner");
    let command = EquationCommand::SetDirected(set_directed::SetDirected { directed: false });
    assert!(equation_command_extent(&command, &decoded).is_some(), "a decoded document must still admit its document verbs");
    assert!(EquationRetainedCommandWork::source_scene(&decoded).is_ok(), "the phase machine reads the same fail-soft projection the extent measures");
}

/// 🧬️ The Actions pane stages `nodeGraphEdit.operations` as a `json_text` argument, which arrives as
/// a `DslValue::String` already holding the JSON document. Re-printing it wrapped the array in
/// quotes and `equation_edit_preflight` refused it as a non-array, so the pane could dispatch no
/// document verb at all. A structured value still prints through `json::to_json_string`.
#[semio_framework_async_macros::async_test]
async fn a_staged_json_text_operations_argument_survives_command_from_action() {
    let staged = dsl::DslValue::Object(vec![("operations".to_string(), dsl::DslValue::String(EQUATION_DEFAULT_EDIT_OPERATIONS.to_string()))]);
    let command = <EquationPlayApp as ArtifactEditor>::command_from_action("nodeGraphEdit", Some(&staged)).expect("staged operations");
    let EquationCommand::NodeGraphEdit(payload) = &command else { panic!("nodeGraphEdit routes to its own command") };
    assert_eq!(payload.operations_json, EQUATION_DEFAULT_EDIT_OPERATIONS);
    assert_eq!(equation_edit_preflight(payload), Some(1), "the staged default is exactly one admitted operation");
    let snapshot = crate::equation_snapshot_with_state(&EquationGraph::default(), &EquationGeometry::default());
    assert!(equation_command_extent(&command, &snapshot).is_some());
}

/// 🧺️ `ArtifactStoreOneItemFootprint::work_items` counts staged edit ROWS, so a point-invertible
/// item costs 2 — the hand-written `1` fail-closed every durable equation gesture with
/// `batched item candidate failed its exact fixed fold contract`.
#[semio_framework_async_macros::async_test]
async fn the_store_preparation_declares_a_point_invertible_footprint() {
    assert_eq!(
        store::ArtifactStoreOneItemFootprint::for_one_invertible_item(store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES).work_items,
        store::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS
    );
    assert_eq!(store::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS, 2);
}
//#endregion 🔖️LoadedDocumentDispatch

```

### ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs

SHA-256 bbe14f703269d0806e73b2aa7141d325f53365ecdd840e8b4d73797f3b6c5731; 24823 bytes.

```rust
//! 🧪️ The editor surface — identity, the two-window manifest, and one dispatch per command.

use crate::editor::wfc2d::{create_wfc2d_editor, Wfc2dEditor, Wfc2dEditorCommand};
// 🔒️ The retained factory is private to the editor module, so the glob `pub use component::*`
// re-export does not carry it — the roster law below reaches it through its defining module.
use crate::editor::wfc2d::component::Wfc2dRetainedCommandJobFactory;
use crate::editor::wfc2d::config::Wfc2dConfig;
use crate::editor::wfc2d::modes::edit::windows::{graph, preview};
use semio_framework_plugin::ArtifactEditor;

fn dispatch(command: &Wfc2dEditorCommand, document: &crate::Wfc2dSnapshot, config: &Wfc2dConfig) -> Result<semio_framework_plugin::Emit<crate::Wfc2dMutation, crate::editor::wfc2d::config::Wfc2dConfigMutation>, semio_framework_plugin::Fault> {
    crate::editor::wfc2d::dispatch(command, document, config, "")
}

#[test]
fn editor_binds_the_artifact_dialect() {
    assert_eq!(Wfc2dEditor::DIALECT.artifact_kind, crate::WFC_2D_DOCUMENT_SCHEMA);
    assert_eq!(Wfc2dEditor::initial_snapshot(), crate::examples::two_room_corridor::document());
}

#[test]
fn editor_manifest_declares_both_windows() {
    let definition = create_wfc2d_editor();
    assert_eq!(definition.id, "s.wfc.wfc2d@1/*#editor");
    let ids: Vec<&str> = definition.window_kinds.iter().map(|window| window.id.as_str()).collect();
    assert!(ids.contains(&graph::WFC_GRAPH_WINDOW));
    assert!(ids.contains(&preview::WFC_2D_PREVIEW_WINDOW));
}

/// ✏️ Every document verb dispatches to exactly one artifact mutation.
#[test]
fn every_document_command_emits_one_mutation() {
    let document = crate::examples::two_room_corridor::document();
    let config = Wfc2dConfig::default();
    let commands = [
        Wfc2dEditorCommand::ChangeSeed { seed: 11 },
        Wfc2dEditorCommand::CreateSlot { id: "room-c".into(), x: 6.0, y: 0.0, width: 2.0, height: 2.0 },
        Wfc2dEditorCommand::DeleteSlot { id: "room-b".into() },
        Wfc2dEditorCommand::MoveSlot { id: "room-a".into(), x: 1.0, y: 1.0 },
        Wfc2dEditorCommand::ResizeSlot { id: "room-a".into(), width: 3.0, height: 3.0 },
        Wfc2dEditorCommand::ConnectSlots { id: "edge-ab".into(), from_slot_id: "room-a".into(), to_slot_id: "room-b".into(), relation: String::new() },
        Wfc2dEditorCommand::DisconnectSlots { id: "edge-a-corridor".into() },
        Wfc2dEditorCommand::PinSlot { id: "room-a".into(), tile_id: String::new() },
        Wfc2dEditorCommand::UnpinSlot { id: "room-a".into() },
        Wfc2dEditorCommand::CreateTile { id: "door".into(), label: None, weight: 1.0 },
        Wfc2dEditorCommand::DeleteTile { id: "room".into() },
        Wfc2dEditorCommand::ChangeTileWeight { tile_id: "room".into(), weight: 4.0 },
        Wfc2dEditorCommand::ChangeTileMedia { tile_id: "room".into() },
        Wfc2dEditorCommand::CreateRule { id: "rule-x".into(), tile_a_id: "room".into(), tile_b_id: "corridor".into(), relation: None, allowed: true },
        Wfc2dEditorCommand::DeleteRule { id: "rule-room-room".into() },
    ];
    for command in &commands {
        let emit = dispatch(command, &document, &config).expect("command dispatches");
        assert_eq!(emit.artifact_mutations.len(), 1, "{command:?} did not emit exactly one mutation");
        assert!(emit.config_mutations.is_empty(), "{command:?} must not touch the pane config");
    }
}

/// 🎥️ Camera and armed-tile verbs go to the PANE config, never to the document lanes.
#[test]
fn view_commands_never_touch_the_document() {
    let document = crate::examples::two_room_corridor::document();
    let config = Wfc2dConfig::default();
    for command in [Wfc2dEditorCommand::ChangeCamera { x: 1.0, y: 2.0, zoom: 3.0 }, Wfc2dEditorCommand::ChangeActiveTile { tile_id: "room".into() }] {
        let emit = dispatch(&command, &document, &config).expect("command dispatches");
        assert!(emit.artifact_mutations.is_empty(), "{command:?} must not emit a document mutation");
        assert_eq!(emit.config_mutations.len(), 1);
    }
}

/// 📌️ A pin with no armed tile and no tiles at all is refused with a domain error, never a pin to
/// an empty id the mutation would then have to reject.
#[test]
fn pinning_without_any_tile_is_refused() {
    let command = Wfc2dEditorCommand::PinSlot { id: "room-a".into(), tile_id: String::new() };
    assert!(dispatch(&command, &crate::Wfc2dSnapshot::default(), &Wfc2dConfig::default()).is_err());
}

/// 🖼️ Both windows render non-empty for every bundled example.
#[test]
fn both_windows_render_for_every_example() {
    let config = Wfc2dConfig::default();
    let transient = crate::editor::wfc2d::transient::Wfc2dTransient::default();
    for document in crate::examples::documents() {
        for body in [graph::WFC_GRAPH_BODY, preview::WFC_2D_PREVIEW_BODY] {
            crate::editor::wfc2d::render_body(body, &document, &config, &transient, None).expect("window renders");
        }
    }
}

/// ⚡️ Every command round-trips through its own binary op encoding.
#[test]
fn commands_round_trip_through_their_binary_codec() {
    let command = Wfc2dEditorCommand::MoveSlot { id: "room-a".into(), x: 1.5, y: -2.5 };
    let bytes = protocol::OpBinary::encode_op(&command).expect("command encodes");
    let decoded: Wfc2dEditorCommand = protocol::OpBinary::decode_op(&bytes).expect("command decodes");
    assert_eq!(decoded, command);
}

/// 🫧️ THE HOST PATH. `render_with_transient` is exactly what `ArtifactEditor::render_with_request_context`
/// runs (it is a one-line delegation), so this drives the real render entry point with a real
/// `TransientView` and proves the solved assignment reaches `wfc-2d-preview` in a running app — the
/// gap the wfc2d audit flagged as blocking. The two arguments left out (`ArtifactInstanceOperationOwnerHandle`
/// and `InteractionView`) are unconstructible from this crate — `InteractionView`'s fields are
/// `pub(crate)` in the framework crate — and neither is read by this artifact's render.
/// 🎨 …and the difference is the tile media, not an incidental label: the solved projection carries
/// one path layer per solved slot, the unsolved one carries none.
/// 🔎️ `ComponentTree` is `Debug` but not `PartialEq`, so the two renders are compared by their
/// debug projection — enough to prove the lane is read, and it fails loudly if it ever is not.
#[test]
fn the_host_render_path_paints_the_solved_assignment() {
    use crate::editor::wfc2d::transient::{Wfc2dAssignment, Wfc2dTransient};
    let document = crate::examples::two_room_corridor::document();
    let config = Wfc2dConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = semio_framework_plugin::ArtifactView::new(&document, &history);
    let cfg = semio_framework_plugin::ConfigView { snapshot: &config, window: None };

    let solved = Wfc2dTransient {
        assignments: vec![
            Wfc2dAssignment { slot_id: "corridor".into(), tile_id: "corridor".into() },
            Wfc2dAssignment { slot_id: "room-a".into(), tile_id: "room".into() },
            Wfc2dAssignment { slot_id: "room-b".into(), tile_id: "room".into() },
        ],
        contradiction: false,
    };
    let empty = Wfc2dTransient::default();

    let painted = crate::editor::wfc2d::render_with_transient(preview::WFC_2D_PREVIEW_BODY, &doc, &cfg, &semio_framework_plugin::TransientView { snapshot: &solved, window: None }).expect("the host render path renders");
    let unsolved = crate::editor::wfc2d::render_with_transient(preview::WFC_2D_PREVIEW_BODY, &doc, &cfg, &semio_framework_plugin::TransientView { snapshot: &empty, window: None }).expect("the host render path renders unsolved too");
    assert_ne!(format!("{painted:?}"), format!("{unsolved:?}"), "the transient lane never reached the preview window: a solved board rendered identically to an unsolved one");

    let solved_layers = preview::preview_layers_json(&document, &solved, None);
    let unsolved_layers = preview::preview_layers_json(&document, &empty, None);
    for slot in &document.slots {
        assert!(solved_layers.contains(&format!("tile-{}-", slot.id)), "slot {} lost its solved tile media", slot.id);
    }
    assert!(!unsolved_layers.contains("\"kind\":\"path\""), "an unsolved, unpinned board must paint no tile media");
}

/// 🖼️ A bitmap tile reaches the canvas as a real PNG data URL through the same host path.
#[test]
fn the_host_render_path_paints_bitmap_tiles_as_pixels() {
    use crate::editor::wfc2d::transient::{Wfc2dAssignment, Wfc2dTransient};
    let document = crate::examples::terrain_ring::document();
    let tile = document.tiles[0].id.clone();
    let transient = Wfc2dTransient { assignments: document.slots.iter().map(|slot| Wfc2dAssignment { slot_id: slot.id.clone(), tile_id: tile.clone() }).collect(), contradiction: false };
    let layers = preview::preview_layers_json(&document, &transient, None);
    assert!(layers.contains("\"kind\":\"image\""), "a bitmap tile must paint an image layer, not a labelled rect");
    assert!(layers.contains("data:image/png;base64,"), "a bitmap tile must carry a real png data url");
    assert!(!layers.contains("bitmap\",\"kind\":\"rect\""), "no bitmap tile may fall back to the outline placeholder here");

    let config = Wfc2dConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = semio_framework_plugin::ArtifactView::new(&document, &history);
    let cfg = semio_framework_plugin::ConfigView { snapshot: &config, window: None };
    crate::editor::wfc2d::render_with_transient(preview::WFC_2D_PREVIEW_BODY, &doc, &cfg, &semio_framework_plugin::TransientView { snapshot: &transient, window: None }).expect("the raster board renders through the host path");
}

//#region 🕹️GraphGestures
/// 🔗️ The shared node-graph record contract (`📺️renderer/🧑‍🎨engine/🧫️fixtures/🧫️node-graph-edit-rows`), as JSON.
fn node_graph_rows_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json"))).expect("the node-graph row fixture is JSON")
}

fn graph_edit(rows: serde_json::Value) -> Result<semio_framework_plugin::Emit<crate::Wfc2dMutation, crate::editor::wfc2d::config::Wfc2dConfigMutation>, semio_framework_plugin::Fault> {
    dispatch(&Wfc2dEditorCommand::NodeGraphEdit { operations_json: rows.to_string() }, &crate::examples::two_room_corridor::document(), &Wfc2dConfig::default())
}

/// 🔗️ Against the shared contract: every accepted row decodes — `setSlider` and `insertPort` are refused BY NAME, a wfc
/// slot graph has neither inline sliders nor variadic ports — and every refused row (the deleted `setHostSnapshot` and
/// `deleteSelection` included) refuses its whole batch.
#[test]
fn node_graph_rows_follow_the_shared_contract() {
    let fixture = node_graph_rows_fixture();
    for case in fixture["accepted"].as_array().expect("accepted rows") {
        let result = graph_edit(serde_json::json!([case["row"].clone()]));
        match case["row"]["operation"].as_str() {
            Some("setSlider" | "insertPort") => assert_eq!(result.err().map(|fault| fault.code.0), Some("wfc2d.node-graph.row".into()), "{}", case["id"]),
            _ => assert!(result.is_ok(), "{}: {result:?}", case["id"]),
        }
    }
    for case in fixture["refused"].as_array().expect("refused rows") {
        let Err(fault) = graph_edit(serde_json::json!([{ "operation": "disconnect", "synapseId": "edge-a-corridor" }, case["row"].clone()])) else { panic!("{} must refuse its batch", case["id"]) };
        assert_eq!(fault.code.0, "wfc2d.node-graph.row", "{}", case["id"]);
    }
}

/// ✋️ A released drag (the `move` record) lands ONE relative `drag-slots` leaf over every moved slot, in document units;
/// an absolute move row is no record and is refused by name.
#[test]
fn a_journaled_drag_record_lands_one_relative_leaf_and_a_malformed_one_is_refused() {
    let scale = crate::editor::wfc2d::WFC_2D_GRAPH_VIEW_SCALE;
    let emit = graph_edit(serde_json::json!([{ "operation": "move", "gestureId": "node-drag:1", "nodeIds": ["room-a", "room-b", "ghost"], "dx": 2.0 * scale, "dy": -0.5 * scale }])).expect("a record dispatches");
    assert_eq!(emit.artifact_mutations, vec![crate::mutations::drag_slots(vec!["room-a".into(), "room-b".into()], 2.0, -0.5)], "slots only, offset in document units");
    let Err(fault) = graph_edit(serde_json::json!([{ "operation": "move", "nodeId": "room-a", "x": 1.0, "y": 2.0 }])) else { panic!("an absolute move row is no gesture record") };
    assert_eq!(fault.code.0, "wfc2d.node-graph.row");
}

/// 📐️ Two records of one batch moving nodes by DIFFERENT offsets are two leaves, still ONE edit.
#[test]
fn two_records_moving_by_different_offsets_are_one_edit_of_two_leaves() {
    let scale = crate::editor::wfc2d::WFC_2D_GRAPH_VIEW_SCALE;
    let emit = graph_edit(serde_json::json!([
        { "operation": "move", "gestureId": "align:1", "nodeIds": ["room-a"], "dx": 0.0, "dy": scale },
        { "operation": "move", "gestureId": "align:1", "nodeIds": ["corridor"], "dx": 0.0, "dy": 2.0 * scale }
    ]))
    .expect("an align dispatches");
    assert_eq!(emit.artifact_mutations, vec![crate::mutations::drag_slots(vec!["room-a".into()], 0.0, 1.0), crate::mutations::drag_slots(vec!["corridor".into()], 0.0, 2.0)]);
}

/// 🧾️ With command authority (an admission's seed) the gesture is ONE `ToolTransaction` of `<appId>#nodeGraphEdit`;
/// two gestures are two transactions; a gesture that moves nothing leaves zero trace.
#[test]
fn a_seeded_gesture_is_one_tool_transaction_and_nothing_moved_is_zero_trace() {
    let document = crate::examples::two_room_corridor::document();
    let scale = crate::editor::wfc2d::WFC_2D_GRAPH_VIEW_SCALE;
    let record = |gesture: &str, dx: f64| format!("[{{\"operation\":\"move\",\"gestureId\":\"{gesture}\",\"nodeIds\":[\"room-a\"],\"dx\":{},\"dy\":0.0}}]", dx * scale);
    let first = crate::editor::wfc2d::dispatch(&Wfc2dEditorCommand::NodeGraphEdit { operations_json: record("node-drag:1", 1.0) }, &document, &Wfc2dConfig::default(), "seed-one").expect("the drag dispatches");
    let transaction = first.transaction.clone().expect("the release is a tool transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool == "s.wfc.wfc2d@1/*#editor#nodeGraphEdit", "{transaction:?}");
    let second = crate::editor::wfc2d::dispatch(&Wfc2dEditorCommand::NodeGraphEdit { operations_json: record("node-drag:2", 1.0) }, &document, &Wfc2dConfig::default(), "seed-two").expect("the second drag dispatches");
    assert_ne!(second.transaction.expect("second ref").id, transaction.id, "two gestures are two transactions");
    let idle = crate::editor::wfc2d::dispatch(&Wfc2dEditorCommand::NodeGraphEdit { operations_json: record("node-drag:3", 0.0) }, &document, &Wfc2dConfig::default(), "seed-three").expect("a release that moved nothing dispatches");
    assert!(idle.artifact_mutations.is_empty() && idle.transaction.is_none(), "a release that moved nothing leaves zero trace");
}

/// 🔗️ A wire drawn between two slots lands as ONE `connect-slots` with a fresh edge id; one already there leaves nothing.
#[test]
fn a_drawn_wire_lands_exactly_one_connect_slots() {
    let emit = graph_edit(serde_json::json!([{ "operation": "connect", "sourceNodeId": "room-a", "sourcePortId": "adjacent-out", "targetNodeId": "room-b", "targetPortId": "adjacent-in" }])).expect("a wire dispatches");
    assert!(matches!(emit.artifact_mutations.as_slice(), [crate::Wfc2dMutation::ConnectSlots(wire)] if wire.edge.from_slot_id == "room-a" && wire.edge.to_slot_id == "room-b"), "{:?}", emit.artifact_mutations);
    let again = graph_edit(serde_json::json!([{ "operation": "connect", "sourceNodeId": "corridor", "sourcePortId": "adjacent-out", "targetNodeId": "room-a", "targetPortId": "adjacent-in" }])).expect("a wire dispatches");
    assert!(again.artifact_mutations.is_empty(), "room-a already borders the corridor, in either direction");
}

/// ✂️ A cut wire lands as ONE `disconnect-slots` naming the document's own edge id.
#[test]
fn a_cut_wire_lands_exactly_one_disconnect_slots() {
    let emit = graph_edit(serde_json::json!([{ "operation": "disconnect", "synapseId": "edge-a-corridor" }])).expect("a cut dispatches");
    assert_eq!(emit.artifact_mutations, vec![crate::mutations::disconnect_slots("edge-a-corridor".into())]);
}

/// 🗑️ A delete row cuts the named adjacencies and deletes the named slots (each cascading its own wires), once each.
#[test]
fn a_delete_row_cuts_its_wires_and_deletes_its_slots() {
    let emit = graph_edit(serde_json::json!([{ "operation": "delete", "nodeIds": ["room-b", "ghost"], "synapseIds": ["edge-a-corridor", "edge-a-corridor-ghost"] }])).expect("a delete dispatches");
    assert_eq!(emit.artifact_mutations, vec![crate::mutations::disconnect_slots("edge-a-corridor".into()), crate::mutations::delete_slot("room-b".into())]);
}

/// 🧘 An empty batch (a host abort) is not an edit at all.
#[test]
fn an_empty_batch_is_no_edit() {
    assert!(graph_edit(serde_json::json!([])).expect("an empty batch dispatches").artifact_mutations.is_empty());
}

/// 🛂️ A verb aimed at an id the document does not hold is refused BY NAME rather than minting an edit
/// against nothing — the palette's argument defaults are static and outlive the example they were
/// authored against.
#[test]
fn a_verb_against_an_unknown_id_is_refused_by_name() {
    let document = crate::examples::two_room_corridor::document();
    for (command, code) in [
        (Wfc2dEditorCommand::DeleteSlot { id: "nope".into() }, "wfc2d.slot.unknown-slot"),
        (Wfc2dEditorCommand::UnpinSlot { id: "nope".into() }, "wfc2d.slot.unknown-slot"),
        (Wfc2dEditorCommand::DeleteTile { id: "nope".into() }, "wfc2d.tile.unknown-tile"),
        (Wfc2dEditorCommand::DisconnectSlots { id: "nope".into() }, "wfc2d.edge.unknown-edge"),
        (Wfc2dEditorCommand::DeleteRule { id: "nope".into() }, "wfc2d.rule.unknown-rule"),
        (Wfc2dEditorCommand::CreateSlot { id: "room-a".into(), x: 0.0, y: 0.0, width: 1.0, height: 1.0 }, "wfc2d.id.taken"),
    ] {
        let Err(fault) = dispatch(&command, &document, &Wfc2dConfig::default()) else { panic!("an unknown id must be refused: {command:?}") };
        assert_eq!(fault.code.0, code, "{command:?}");
    }
}

/// 🏁 The solve the preview paints is the artifact's OWN inference, run to completion.
#[test]
fn the_solve_verb_answers_a_transient_assignment() {
    let document = crate::examples::two_room_corridor::document();
    let mutations = crate::editor::wfc2d::solve_transient(&document).expect("the boot example solves");
    assert_eq!(mutations.len(), 1, "one solve is one transient publication");
    let crate::editor::wfc2d::transient::Wfc2dTransientMutation::SetSolve(solve) = &mutations[0];
    let transient = crate::editor::wfc2d::transient::Wfc2dTransient { assignments: solve.assignments.clone(), contradiction: solve.contradiction };
    assert!(!transient.contradiction, "the boot example is satisfiable");
    assert_eq!(transient.assignments.len(), document.slots.len(), "every slot is assigned");
    for row in &transient.assignments {
        assert!(document.tiles.iter().any(|tile| tile.id == row.tile_id), "slot {} was assigned an undeclared tile", row.slot_id);
    }
}

/// 🗃️ The example picker answers a whole-document LOAD, never a mutation set — which is exactly why
/// re-picking the booted example mints no undo entry.
#[test]
fn the_example_picker_loads_a_document_instead_of_editing_one() {
    let document = crate::examples::two_room_corridor::document();
    for example_id in [crate::examples::two_room_corridor::ID, crate::examples::wall_roof_facade_strip::ID, crate::examples::hex_ring::ID, crate::examples::terrain_ring::ID] {
        let emit = dispatch(&Wfc2dEditorCommand::SetActiveExample { example_id: example_id.to_string() }, &document, &Wfc2dConfig::default()).expect("every offered example loads");
        assert!(emit.artifact_mutations.is_empty(), "an example load is not a document edit");
        assert!(matches!(emit.effects.first(), Some(semio_framework::kernel::Effect::LoadDocument { .. })), "an example load is one LoadDocument effect");
    }
    let Err(fault) = dispatch(&Wfc2dEditorCommand::SetActiveExample { example_id: "not-an-example".into() }, &document, &Wfc2dConfig::default()) else { panic!("an unknown example must be refused") };
    assert_eq!(fault.code.0, "wfc2d.example.unknown");
}
//#endregion 🕹️GraphGestures

/// ⚖️ LAW: this app's one app-owned factory carries ONE roster — `TOOL_IDS`, its
/// `PUBLICATION_CONTRACTS` and its `bounded_first_step_tool_proofs!` rows name exactly the same
/// tools. The framework refuses app registration outright when they drift
/// (`interactive-job.publication-contract` when a lane contract names an unowned tool,
/// `interactive-job.catalog-incomplete` when a migrated command has no owner-local proof), and that
/// refusal is a guest-side `panic!` — so one missing row aborted the whole wfc component at boot and
/// every one of its panes reached `data-shell-error` instead of `data-shell-ready`.
#[test]
fn the_owned_factory_tool_ids_publication_contracts_and_proofs_are_one_exact_roster() {
    use semio_framework_plugin::ArtifactOwnedToolJobFactory;
    let tools: std::collections::BTreeSet<&str> = crate::editor::wfc2d::WFC_2D_RETAINED_TOOL_IDS.iter().copied().collect();
    let publication: std::collections::BTreeSet<&str> = <Wfc2dRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter().map(|contract| contract.tool_id).collect();
    assert_eq!(publication, tools, "every owned tool declares exactly one publication-lane contract");
    for contract in <Wfc2dRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS {
        assert!(!contract.lanes.is_empty(), "tool {} declares no publication lane", contract.tool_id);
    }
    let proofs: std::collections::BTreeSet<&str> = <Wfc2dEditor as semio_framework_plugin::ArtifactEditor>::bounded_first_step_tool_proofs().iter().map(|proof| proof.tool_id()).collect();
    assert_eq!(proofs, tools, "every owned tool carries its owner-local bounded reducer proof");
}

/// ⚖️ LAW: every retained tool is a DECLARED `Migrated` action of the BUILT manifest. The framework
/// computes `expected = TOOL_JOB_IDS ∩ migrated_tool_ids(definition)` and refuses any
/// `bounded_first_step_tool_proofs!` row outside it with `interactive-job.catalog-authority` — a
/// guest-side `panic!` at app registration that aborts the whole wfc component, so every wfc pane
/// lands on `data-shell-error`. `commit-fill` reached the roster, the lane contracts and the proofs
/// without any window kind or app-level action ever declaring it, which is exactly that refusal.
/// `window_kind_actions` is the same join the live `AppActionRegistry::from_definition` performs
/// (a window's own rows plus the app-level roster no window claims).
#[test]
fn every_retained_tool_is_a_declared_migrated_action_of_the_built_manifest() {
    let definition = create_wfc2d_editor();
    let declared: std::collections::BTreeMap<String, semio_framework::InteractiveJobClassification> = definition
        .window_kinds
        .iter()
        .flat_map(|window| semio_framework::window_kind_actions(&definition, window))
        .map(|action| (action.id.clone(), action.semantics.execution.interactive_job))
        .collect();
    for tool_id in crate::editor::wfc2d::WFC_2D_RETAINED_TOOL_IDS {
        assert_eq!(
            declared.get(*tool_id),
            Some(&semio_framework::InteractiveJobClassification::Migrated),
            "retained tool '{tool_id}' is declared by no window kind and no app action, so the framework refuses its proof row"
        );
    }
}

/// 🎯️ LAW: the editor declares the artifact kind it edits (the artifact's own `artifact_kind()`), which is
/// what the hub's one open-target rule (`app_opens_kind`, `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs`)
/// pairs with this editor and the viewer of its dialect — so a WFC 2D rule set can be created and opened as a hub document.
#[test]
fn the_editor_declares_the_artifact_kind_it_edits() {
    assert_eq!(create_wfc2d_editor().artifact_kinds, vec![crate::artifact_kind()]);
}

semio_framework_plugin::history_edit_acceptance_law!("wfc", Wfc2dEditor, || semio_framework_plugin::App { definition: create_wfc2d_editor(), examples: Vec::new() }, "../..");

```

### ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs

SHA-256 1d6ef879a33ad6822918933d90d47c7a7353abfc60ded9d21925d6c101db9cf7; 23446 bytes.

```rust
//! 🧪️ Editor surface laws: the manifest is canonical, every command dispatches to the mutation it
//! names, camera and armed-tile verbs never touch the document, and both windows render.

use super::*;
use crate::mutations::drag_slots;
use semio_framework_plugin::ArtifactEditor;

fn document() -> Wfc3dSnapshot {
    crate::examples::two_room_corridor::snapshot()
}

/// ✏️ `ArtifactView`/`ConfigView` carry private fields, so nothing outside the framework can build
/// one — the editor's own pure `command_emit`/`render_body` split is what makes the mapping and the
/// per-window render reachable from here, and `handle`/`render` are one-line adapters over them.
fn dispatch(command: &Wfc3dEditorCommand, snapshot: &Wfc3dSnapshot, config: &Wfc3dConfig) -> Result<Emit<Wfc3dMutation, Wfc3dConfigMutation>, Fault> {
    command_emit(command, snapshot, config, "")
}

#[test]
fn the_editor_app_id_is_the_canonical_surface_id() {
    let definition = create_wfc3d_editor();
    assert_eq!(definition.id, "s.wfc.wfc3d@1/*#editor");
    assert_eq!(definition.breadcrumb, vec!["semio".to_string(), "wfc".to_string(), "3d".to_string()]);
}

/// 🪟️ Two windows, one layout: the problem on the left, its solution on the right.
#[test]
fn the_editor_declares_the_graph_and_the_preview_window() {
    let definition = create_wfc3d_editor();
    let ids: Vec<&str> = definition.window_kinds.iter().map(|window| window.id.as_str()).collect();
    assert_eq!(ids, vec![graph::WFC_GRAPH_WINDOW, preview::WFC_3D_PREVIEW_WINDOW]);
}

#[test]
fn the_editor_boots_on_a_real_document() {
    assert_eq!(<Wfc3dEditor as ArtifactEditor>::initial_snapshot(), document());
    assert_eq!(<Wfc3dEditor as ArtifactEditor>::DIALECT, WFC3D_DIALECT);
}

/// ✏️ One command, one mutation — every user-facing verb reaches the schema tree's own builder.
#[test]
fn every_document_command_dispatches_to_exactly_one_mutation() {
    let document = document();
    let config = Wfc3dConfig::default();
    let commands = vec![
        Wfc3dEditorCommand::ChangeSeed { seed: 11 },
        Wfc3dEditorCommand::CreateSlot { id: "room-c".into(), x: 3.0, y: 0.0, z: 0.0, width: 1.0, height: 1.0, depth: 1.0 },
        Wfc3dEditorCommand::DeleteSlot { id: "room-b".into() },
        Wfc3dEditorCommand::MoveSlot { id: "room-b".into(), x: 2.0, y: 1.0, z: Some(0.0) },
        Wfc3dEditorCommand::ResizeSlot { id: "room-a".into(), width: 2.0, height: 1.0, depth: Some(1.0) },
        Wfc3dEditorCommand::ConnectSlots { id: "edge-a-b".into(), from_slot_id: "room-a".into(), to_slot_id: "room-b".into(), relation: String::new() },
        Wfc3dEditorCommand::DisconnectSlots { id: "edge-corridor-b".into() },
        Wfc3dEditorCommand::PinSlot { id: "room-a".into(), tile_id: String::new() },
        Wfc3dEditorCommand::UnpinSlot { id: "room-a".into() },
        Wfc3dEditorCommand::CreateTile { id: "stair".into(), label: None, weight: 0.0 },
        Wfc3dEditorCommand::DeleteTile { id: "corridor".into() },
        Wfc3dEditorCommand::ChangeTileWeight { tile_id: "room".into(), weight: 4.0 },
        Wfc3dEditorCommand::ChangeTileMedia { tile_id: "room".into() },
        Wfc3dEditorCommand::CreateRule { id: "rule-x".into(), tile_a_id: "room".into(), tile_b_id: "corridor".into(), relation: None, allowed: false },
        Wfc3dEditorCommand::DeleteRule { id: "rule-room-room".into() },
    ];
    assert_eq!(commands.len(), crate::mutations::KINDS.len(), "every document mutation kind has a command");
    for command in commands {
        let emit = match dispatch(&command, &document, &config) {
            Ok(emit) => emit,
            Err(fault) => panic!("{command:?} must dispatch: {fault:?}"),
        };
        assert_eq!(emit.artifact_mutations.len(), 1, "{command:?} must emit exactly one mutation");
        assert!(emit.config_mutations.is_empty(), "{command:?} must not touch the pane config");
    }
}

/// 🎥️ The two view verbs go to the PER-PANE config and never to the document, so a camera move can
/// never enter the undo history.
#[test]
fn the_view_verbs_touch_the_config_and_never_the_document() {
    let document = document();
    let config = Wfc3dConfig::default();
    for command in [Wfc3dEditorCommand::ChangeCamera { x: 1.0, y: 2.0, zoom: 3.0 }, Wfc3dEditorCommand::ChangeActiveTile { tile_id: "room".into() }] {
        let emit = match dispatch(&command, &document, &config) {
            Ok(emit) => emit,
            Err(fault) => panic!("{command:?} must dispatch: {fault:?}"),
        };
        assert!(emit.artifact_mutations.is_empty(), "{command:?} must author no document mutation");
        assert_eq!(emit.config_mutations.len(), 1);
    }
}

/// 🀄️ A pin with no explicit tile uses the PANE's armed tile; with none armed it falls back to the
/// document's first tile rather than emitting an id the mutation would refuse.
#[test]
fn a_pin_with_no_explicit_tile_uses_the_panes_armed_tile() {
    let document = document();
    let armed = Wfc3dConfig { active_tile_id: "room".into(), ..Default::default() };
    let emit = dispatch(&Wfc3dEditorCommand::PinSlot { id: "room-a".into(), tile_id: String::new() }, &document, &armed).unwrap_or_else(|_| panic!("pin dispatches"));
    assert_eq!(emit.artifact_mutations, vec![pin_slot("room-a".into(), "room".into())]);
    let emit = dispatch(&Wfc3dEditorCommand::PinSlot { id: "room-a".into(), tile_id: String::new() }, &document, &Wfc3dConfig::default()).unwrap_or_else(|_| panic!("pin dispatches"));
    assert_eq!(emit.artifact_mutations, vec![pin_slot("room-a".into(), "corridor".into())], "the fallback is the document's FIRST tile");
}

#[test]
fn a_pin_into_an_empty_catalogue_is_a_clear_domain_fault_not_a_refused_mutation() {
    let mut document = document();
    document.tiles.clear();
    document.rules.clear();
    let fault = match dispatch(&Wfc3dEditorCommand::PinSlot { id: "room-a".into(), tile_id: String::new() }, &document, &Wfc3dConfig::default()) {
        Ok(_) => panic!("a pin into an empty catalogue must fault"),
        Err(fault) => fault,
    };
    assert!(format!("{fault:?}").contains("wfc3d.tile.unknown-pin"));
}

/// 📐️ A zero extent from a form default is substituted with the unit value, so the editor never
/// emits a command its own invariant guard would refuse.
#[test]
fn a_zero_extent_command_is_normalised_to_a_unit_box() {
    let document = document();
    let emit = dispatch(&Wfc3dEditorCommand::CreateSlot { id: "room-c".into(), x: 0.0, y: 0.0, z: 0.0, width: 0.0, height: 0.0, depth: 0.0 }, &document, &Wfc3dConfig::default()).unwrap_or_else(|_| panic!("dispatch"));
    let (applied, _) = vcs::apply_mutation(&document, &emit.artifact_mutations[0]).expect("the normalised slot applies");
    let slot = applied.slots.iter().find(|slot| slot.id == "room-c").expect("room-c exists");
    assert_eq!((slot.width, slot.height, slot.depth), (1.0, 1.0, 1.0));
}

/// 🔤️ A created row lands at its collection's canonical sorted position, so an undo restores it there.
#[test]
fn a_created_row_lands_at_its_canonical_sorted_position() {
    let document = document();
    let emit = dispatch(&Wfc3dEditorCommand::CreateTile { id: "attic".into(), label: None, weight: 1.0 }, &document, &Wfc3dConfig::default()).unwrap_or_else(|_| panic!("dispatch"));
    let (applied, _) = vcs::apply_mutation(&document, &emit.artifact_mutations[0]).expect("apply");
    assert_eq!(applied.tiles.first().map(|tile| tile.id.as_str()), Some("attic"), "\"attic\" sorts before \"corridor\"");
}

#[test]
fn both_window_bodies_render_for_every_example() {
    let config = Wfc3dConfig::default();
    let transient = crate::editor::wfc3d::transient::Wfc3dTransient::default();
    for document in [crate::examples::two_room_corridor::snapshot(), crate::examples::wall_roof_facade_strip::snapshot(), crate::examples::tower_stack::snapshot()] {
        for body in [graph::WFC_GRAPH_BODY, preview::WFC_3D_PREVIEW_BODY] {
            let tree = render_body(body, &document, &config, &transient, None).unwrap_or_else(|error| panic!("{body} must render: {error:?}"));
            assert!(!format!("{tree:?}").is_empty());
        }
    }
}

#[test]
fn an_unknown_body_key_renders_a_label_instead_of_failing() {
    assert!(render_body("nope", &document(), &Wfc3dConfig::default(), &crate::editor::wfc3d::transient::Wfc3dTransient::default(), None).is_ok());
}

/// 🕸️ The graph view drops `z` deliberately: the canvas is a plan of an arbitrary graph, and the
/// third axis belongs to the preview pane. It also SCALES into canvas units, because a slot is a box
/// in metres and a one-metre node at viewport zoom 1 is one pixel.
#[test]
fn the_graph_view_projects_x_and_y_scaled_and_drops_z() {
    use crate::editor::wfc3d::modes::edit::windows::graph::SlotGraphView;
    let document = crate::examples::tower_stack::snapshot();
    let slots = Wfc3dGraphView(&document).graph_slots();
    assert_eq!(slots.len(), document.slots.len());
    let cantilever = slots.iter().find(|slot| slot.id == "cantilever").expect("the cantilever projects");
    assert_eq!((cantilever.x, cantilever.y), (1.5 * WFC_3D_GRAPH_UNIT, 3.0 * WFC_3D_GRAPH_UNIT));
}

/// ✋️ A released node drag — the node-graph gesture record — is ONE relative `drag-slots` in DOCUMENT units with
/// `dz = 0`: the canvas never saw the third axis, so a drag cannot flatten a stack. An absolute `move` row is no
/// gesture record and is refused by name.
#[test]
fn a_dragged_node_lands_one_drag_slots_in_document_units_keeping_z() {
    let document = crate::examples::tower_stack::snapshot();
    let authored = document.slots.iter().find(|slot| slot.id == "cantilever").expect("the cantilever exists").clone();
    let operations = format!(r#"[{{"operation":"move","gestureId":"node-drag:1","nodeIds":["cantilever"],"dx":{},"dy":{}}}]"#, 2.5 * WFC_3D_GRAPH_UNIT, 2.0 * WFC_3D_GRAPH_UNIT);
    let mutations = graph_edit_mutations(&document, &operations).expect("the gesture lowers");
    assert_eq!(mutations.len(), 1, "one gesture is one edit, never one per pointer tick");
    assert_eq!(mutations, vec![drag_slots(vec!["cantilever".into()], 2.5, 2.0, 0.0)]);
    let mut moved = document.clone();
    crate::mutations::apply_wfc3d_mutation(&mut moved, &mutations[0]).expect("the drag applies");
    let landed = moved.slots.iter().find(|slot| slot.id == "cantilever").expect("the cantilever survives its drag");
    assert_eq!((landed.x, landed.y, landed.z), (authored.x + 2.5, authored.y + 2.0, authored.z), "z is kept");
    let absolute = format!(r#"[{{"operation":"move","nodeId":"cantilever","x":{},"y":{}}}]"#, 4.0 * WFC_3D_GRAPH_UNIT, 5.0 * WFC_3D_GRAPH_UNIT);
    let Err(fault) = graph_edit_mutations(&document, &absolute) else { panic!("an absolute move row is no gesture record") };
    assert_eq!(fault.code.0, "wfc3d.node-graph.row");
}

/// 🧾️ With command authority (an admission's seed) a release is ONE `ToolTransaction` of `<appId>#nodeGraphEdit`;
/// two releases are two transactions; one that moves nothing leaves zero trace.
#[test]
fn a_seeded_release_is_one_tool_transaction_and_nothing_moved_is_zero_trace() {
    let document = crate::examples::two_room_corridor::snapshot();
    let record = |gesture: &str, dx: f64| Wfc3dEditorCommand::NodeGraphEdit { operations_json: format!(r#"[{{"operation":"move","gestureId":"{gesture}","nodeIds":["room-a"],"dx":{},"dy":0.0}}]"#, dx * WFC_3D_GRAPH_UNIT) };
    let first = command_emit(&record("node-drag:1", 1.0), &document, &Wfc3dConfig::default(), "seed-one").expect("the drag dispatches");
    let transaction = first.transaction.clone().expect("the release is a tool transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool == "s.wfc.wfc3d@1/*#editor#nodeGraphEdit", "{transaction:?}");
    let second = command_emit(&record("node-drag:2", 1.0), &document, &Wfc3dConfig::default(), "seed-two").expect("the second drag dispatches");
    assert_ne!(second.transaction.expect("second ref").id, transaction.id, "two releases are two transactions");
    let idle = command_emit(&record("node-drag:3", 0.0), &document, &Wfc3dConfig::default(), "seed-three").expect("an idle release dispatches");
    assert!(idle.artifact_mutations.is_empty() && idle.transaction.is_none(), "a release that moved nothing leaves zero trace");
}

/// 🔗 A connect gesture between two slot nodes mints ONE deterministic edge; repeating it is a
/// no-op, because a duplicate edge id is a fatal mutation invariant, not a second edge.
#[test]
fn a_connect_gesture_lands_one_connect_slots_and_never_duplicates() {
    let document = crate::examples::two_room_corridor::snapshot();
    let operations = r#"[{"operation":"connect","sourceNodeId":"room-a","sourcePortId":"room-a@adjacent-out","targetNodeId":"room-b","targetPortId":"room-b@adjacent-in"}]"#;
    let mutations = graph_edit_mutations(&document, operations).expect("the gesture lowers");
    assert!(matches!(mutations.as_slice(), [Wfc3dMutation::ConnectSlots(wire)] if wire.edge.from_slot_id == "room-a" && wire.edge.to_slot_id == "room-b"), "{mutations:?}");
    let repeated = r#"[{"operation":"connect","sourceNodeId":"room-a","sourcePortId":"adjacent-out","targetNodeId":"corridor","targetPortId":"adjacent-in"}]"#;
    assert!(graph_edit_mutations(&document, repeated).expect("the gesture lowers").is_empty(), "room-a already borders the corridor, so the gesture authors nothing");
}

/// 🎨️ Every bundled example resolves by its registered id, the empty id is the shell's own "default
/// document", and an unregistered id faults instead of opening a blank document.
#[test]
fn set_active_example_resolves_every_registered_example_and_refuses_the_rest() {
    for id in WFC_3D_EXAMPLE_IDS {
        assert_eq!(example_snapshot(id).expect("a registered example loads").schema, WFC3D_DOCUMENT_SCHEMA);
    }
    assert_eq!(example_snapshot("").expect("the empty id is the boot document"), crate::examples::two_room_corridor::snapshot());
    assert!(example_snapshot("nope").is_err(), "an unregistered example id must fault, not load a blank document");
}

/// 🎯️ Every action the editor manifest declares must bridge through `command_from_action`, or the
/// shell's Actions pane, its example picker and the canvas gestures all answer `dispatch-failed`.
#[test]
fn every_declared_action_bridges_through_command_from_action() {
    let definition = create_wfc3d_editor();
    let skip = [
        "undo",
        "redo",
        "commitCheckpoint",
        "createAlternative",
        "switchAlternative",
        "checkoutCheckpoint",
        "copy",
        "cut",
        "paste",
        "revertToCommand",
        "setHistoryCommandFilter",
        "noteShellCommand",
        "recordTutorial",
        "startIntroduction",
        "startTutorial",
        "setActiveUtility",
        "setActiveTool",
        "interactionSelect",
        "interactionHover",
        "clearSelection",
        "selectAll",
        "setSelectionMode",
        "setInteractionGranularity",
        "toolRunStart",
        "toolRunPause",
        "toolRunResume",
        "toolRunStep",
        "toolRunAbort",
        "toolRunFinalize",
        "toolRunDismiss",
        "commit-fill",
    ];
    let mut bridged = 0;
    for window in &definition.window_kinds {
        for action in &window.actions {
            if skip.contains(&action.id.as_str()) {
                continue;
            }
            <Wfc3dEditor as semio_framework_plugin::ArtifactEditor>::command_from_action(&action.id, None).unwrap_or_else(|error| panic!("action {} failed to bridge: {}", action.id, error.message));
            bridged += 1;
        }
    }
    assert!(bridged >= 10, "expected every declared wfc3d verb to bridge, saw {bridged}");
}

/// 🔗️ Against the shared node-graph record contract (`📺️renderer/🧑‍🎨engine/🧫️fixtures/🧫️node-graph-edit-rows`): every
/// accepted row decodes — `setSlider` and `insertPort` are refused BY NAME, a wfc slot graph has neither inline sliders
/// nor variadic ports — and every refused row (the deleted `setHostSnapshot` and `deleteSelection` included) refuses its
/// whole batch.
#[test]
fn node_graph_rows_follow_the_shared_contract() {
    let document = crate::examples::two_room_corridor::snapshot();
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json"))).expect("the node-graph row fixture is JSON");
    for case in fixture["accepted"].as_array().expect("accepted rows") {
        let result = graph_edit_mutations(&document, &serde_json::json!([case["row"].clone()]).to_string());
        match case["row"]["operation"].as_str() {
            Some("setSlider" | "insertPort") => assert_eq!(result.err().map(|fault| fault.code.0), Some("wfc3d.node-graph.row".into()), "{}", case["id"]),
            _ => assert!(result.is_ok(), "{}: {result:?}", case["id"]),
        }
    }
    for case in fixture["refused"].as_array().expect("refused rows") {
        let batch = serde_json::json!([{ "operation": "disconnect", "synapseId": document.edges[0].id }, case["row"].clone()]);
        let Err(fault) = graph_edit_mutations(&document, &batch.to_string()) else { panic!("{} must refuse its batch", case["id"]) };
        assert_eq!(fault.code.0, "wfc3d.node-graph.row", "{}", case["id"]);
    }
}

/// ✂️🗑️ A cut wire is ONE `disconnect-slots` of the document's own edge id; a delete row cuts the named adjacencies and
/// deletes the named slots (each cascading its own wires), once each, and never cuts a wire twice.
#[test]
fn cut_and_delete_rows_land_their_intent_leaves() {
    let document = crate::examples::two_room_corridor::snapshot();
    let edge = document.edges[0].id.clone();
    let cut = graph_edit_mutations(&document, &serde_json::json!([{ "operation": "disconnect", "synapseId": edge }]).to_string()).expect("a cut lowers");
    assert_eq!(cut, vec![disconnect_slots(edge.clone())]);
    let deleted = graph_edit_mutations(&document, &serde_json::json!([{ "operation": "delete", "nodeIds": ["room-b", "ghost"], "synapseIds": [edge, "ghost-wire"] }, { "operation": "disconnect", "synapseId": document.edges[0].id }]).to_string()).expect("a delete lowers");
    assert_eq!(deleted, vec![disconnect_slots(document.edges[0].id.clone()), delete_slot("room-b".into())]);
}

/// 🚚️ The `wfc-graph` window is shared with `wfc2d`, so its staged `move-slot`/`resize-slot` forms
/// offer only `x`/`y` and `width`/`height`. A dispatch that omits the third axis must KEEP the slot's
/// authored `z`/`depth` — a 2d form must not be able to flatten a stack it cannot see.
#[test]
fn a_two_axis_move_or_resize_keeps_the_authored_third_axis() {
    let document = crate::examples::tower_stack::snapshot();
    let authored = document.slots.iter().find(|slot| slot.id == "cantilever").expect("the cantilever exists").clone();
    let config = Wfc3dConfig::default();

    let moved = dispatch(&Wfc3dEditorCommand::MoveSlot { id: "cantilever".into(), x: 9.0, y: 9.0, z: None }, &document, &config).expect("move dispatches");
    assert_eq!(moved.artifact_mutations, vec![move_slot("cantilever".into(), 9.0, 9.0, authored.z)]);

    let resized = dispatch(&Wfc3dEditorCommand::ResizeSlot { id: "cantilever".into(), width: 4.0, height: 5.0, depth: None }, &document, &config).expect("resize dispatches");
    assert_eq!(resized.artifact_mutations, vec![resize_slot("cantilever".into(), 4.0, 5.0, authored.depth)]);
}

/// ⚖️ LAW: this app's one app-owned factory carries ONE roster — `TOOL_IDS`, its
/// `PUBLICATION_CONTRACTS` and its `bounded_first_step_tool_proofs!` rows name exactly the same
/// tools. The framework refuses app registration outright when they drift
/// (`interactive-job.publication-contract` when a lane contract names an unowned tool,
/// `interactive-job.catalog-incomplete` when a migrated command has no owner-local proof), and that
/// refusal is a guest-side `panic!` — so one missing row aborted the whole wfc component at boot and
/// every one of its panes reached `data-shell-error` instead of `data-shell-ready`.
#[test]
fn the_owned_factory_tool_ids_publication_contracts_and_proofs_are_one_exact_roster() {
    use semio_framework_plugin::ArtifactOwnedToolJobFactory;
    let tools: std::collections::BTreeSet<&str> = WFC_3D_RETAINED_TOOL_IDS.iter().copied().collect();
    let publication: std::collections::BTreeSet<&str> = <Wfc3dRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter().map(|contract| contract.tool_id).collect();
    assert_eq!(publication, tools, "every owned tool declares exactly one publication-lane contract");
    for contract in <Wfc3dRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS {
        assert!(!contract.lanes.is_empty(), "tool {} declares no publication lane", contract.tool_id);
    }
    let proofs: std::collections::BTreeSet<&str> = <Wfc3dEditor as semio_framework_plugin::ArtifactEditor>::bounded_first_step_tool_proofs().iter().map(|proof| proof.tool_id()).collect();
    assert_eq!(proofs, tools, "every owned tool carries its owner-local bounded reducer proof");
}

/// ⚖️ LAW: every retained tool is a DECLARED `Migrated` action of the BUILT manifest. The framework
/// computes `expected = TOOL_JOB_IDS ∩ migrated_tool_ids(definition)` and refuses any
/// `bounded_first_step_tool_proofs!` row outside it with `interactive-job.catalog-authority` — a
/// guest-side `panic!` at app registration that aborts the whole wfc component, so every wfc pane
/// lands on `data-shell-error`. `commit-fill` reached the roster, the lane contracts and the proofs
/// without any window kind or app-level action ever declaring it, which is exactly that refusal.
/// `window_kind_actions` is the same join the live `AppActionRegistry::from_definition` performs
/// (a window's own rows plus the app-level roster no window claims).
#[test]
fn every_retained_tool_is_a_declared_migrated_action_of_the_built_manifest() {
    let definition = create_wfc3d_editor();
    let declared: std::collections::BTreeMap<String, semio_framework::InteractiveJobClassification> = definition
        .window_kinds
        .iter()
        .flat_map(|window| semio_framework::window_kind_actions(&definition, window))
        .map(|action| (action.id.clone(), action.semantics.execution.interactive_job))
        .collect();
    for tool_id in WFC_3D_RETAINED_TOOL_IDS {
        assert_eq!(
            declared.get(*tool_id),
            Some(&semio_framework::InteractiveJobClassification::Migrated),
            "retained tool '{tool_id}' is declared by no window kind and no app action, so the framework refuses its proof row"
        );
    }
}

/// 🎯️ LAW: the editor declares the artifact kind it edits (the artifact's own `artifact_kind()`), which is
/// what the hub's one open-target rule (`app_opens_kind`, `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs`)
/// pairs with this editor and the viewer of its dialect — so a WFC 3D rule set can be created and opened as a hub document.
#[test]
fn the_editor_declares_the_artifact_kind_it_edits() {
    assert_eq!(create_wfc3d_editor().artifact_kinds, vec![crate::artifact_kind()]);
}

```

### ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs

SHA-256 16dcf0ef0e7bd6ac01ad0afe8dce713e34cc9c76323eaa2dd713f94005145311; 13522 bytes.

```rust
//! 🕸️ Laws of the generation2d `nodeGraphEdit` verb (design §5, §13.1, §13.3): wires round-trip as one-shot structural
//! edits, a released node drag and a dropped node are ONE tool transaction of the RELATIVE `move-nodes` leaf, a slider
//! press is ONE tool transaction of the ABSOLUTE `change-slider-value` leaf with provisional ticks and a zero-trace
//! cancel, and every gesture row is labelled from its leaf in English and German.

use super::*;
use crate::editor::generation2d::unit_tests::context::{app, close, dispatch, snapshot_read, Generation2dApp};
use crate::editor::generation2d::Generation2dCommand;
use semio_framework::kernel::{HistoryEntry, UiDirtyScope};
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_plugin::artifact_app_laws::{meta, settle_registered_typed_operation};
use semio_framework_plugin::PluginApp;

async fn edit(app: &mut Generation2dApp, operations: serde_json::Value) {
    dispatch(app, Generation2dCommand::NodeGraphEdit(NodeGraphEdit { operations_json: operations.to_string() })).await;
}

/// 🕹️ One `nodeGraphEdit` exactly as a host sends it (`operations` plus a press's top-level `gesture`/`commit`/`abort`),
/// settled through the registered ladder.
async fn send(app: &mut Generation2dApp, args: serde_json::Value) {
    let action_meta = meta("local");
    let args: dsl::DslValue = args.into();
    app.handle_action("nodeGraphEdit", Some(&args), &action_meta).await.expect("nodeGraphEdit admitted");
    settle_registered_typed_operation(app, action_meta.instance_id).await.expect("nodeGraphEdit settles");
}

/// 🧾️ Every applied history row that carries document operations, oldest first.
async fn edit_rows(app: &mut Generation2dApp) -> Vec<HistoryEntry> {
    let mut rows: Vec<HistoryEntry> = PluginApp::history_snapshot(app).await.expect("history").upserts.into_iter().filter(|entry| entry.applied && !entry.op_lines.is_empty()).collect();
    rows.sort_by_key(|entry| entry.seq);
    rows
}

/// 📍️ Lays every widget of the document out on the canvas (the `reorganize` verb, ONE setup edit), so a drag has base
/// positions; answers the placed ids with their base positions.
async fn place_every_widget(app: &mut Generation2dApp) -> Vec<(String, (f64, f64))> {
    dispatch(app, Generation2dCommand::Reorganize(crate::editor::generation2d::commands::reorganize::Reorganize {})).await;
    let read = snapshot_read(app);
    read.host_snapshot.widgets.iter().map(|widget| crate::widget_id(widget).to_string()).filter_map(|id| read.host_snapshot.layout.get(id.as_str()).map(|layout| (layout.x, layout.y)).map(|base| (id, base))).collect()
}

/// 📐️ Whether `id` sits at `(x, y)` up to the rounding of a relative offset re-derived from an absolute drop.
fn sits_at(app: &Generation2dApp, id: &str, (x, y): (f64, f64)) -> bool {
    position(app, id).is_some_and(|(at_x, at_y)| (at_x - x).abs() < 1e-9 && (at_y - y).abs() < 1e-9)
}

/// 🔌️ One live wire of the current document: `(id, fromNode, fromPort, toNode, toPort)`.
fn one_live_wire(host_snapshot: &FlowHostSnapshot) -> (String, String, String, String, String) {
    let synapse = host_snapshot.synapses.first().expect("every bundled example wires at least one synapse");
    (synapse.id.clone(), synapse.from.clone(), synapse.from_port.clone(), synapse.to.clone(), synapse.to_port.clone())
}

fn wire_exists(host_snapshot: &FlowHostSnapshot, from: &str, from_port: &str, to: &str, to_port: &str) -> bool {
    host_snapshot.synapses.iter().any(|synapse| synapse.from == from && synapse.from_port == from_port && synapse.to == to && synapse.to_port == to_port)
}

fn slider_value(host_snapshot: &FlowHostSnapshot, widget_id: &str) -> Option<f64> {
    host_snapshot.widgets.iter().find_map(|widget| match widget {
        semio_framework_artifact_flow_flow::Widget::InputSlider { id, value, .. } if id == widget_id => Some(*value),
        _ => None,
    })
}

fn position(app: &Generation2dApp, id: &str) -> Option<(f64, f64)> {
    snapshot_read(app).host_snapshot.layout.get(id).map(|layout| (layout.x, layout.y))
}

fn english(entry: &HistoryEntry) -> String {
    entry.label.resolve(protocol::Terminology::Native, protocol::Locale::En).to_string()
}

fn german(entry: &HistoryEntry) -> String {
    entry.label.resolve(protocol::Terminology::Native, protocol::Locale::De).to_string()
}

/// ⚖️ LAW: `disconnect` cuts a wire and `connect` draws it again between the same two ports — the demo's neurons declare
/// their input ports, so the bare host resolves the endpoints with no operator installed.
#[semio_framework_async_macros::async_test]
async fn disconnect_then_connect_round_trips_one_wire() {
    let mut app = app().await;
    let (before, synapse_id, from, from_port, to, to_port) = {
        let read = snapshot_read(&app);
        let (id, from, from_port, to, to_port) = one_live_wire(&read.host_snapshot);
        (read.host_snapshot.synapses.len(), id, from, from_port, to, to_port)
    };
    edit(&mut app, serde_json::json!([{ "operation": "disconnect", "synapseId": synapse_id }])).await;
    {
        let read = snapshot_read(&app);
        assert_eq!(read.host_snapshot.synapses.len(), before - 1, "disconnect must remove exactly the one wire it names");
        assert!(!wire_exists(&read.host_snapshot, &from, &from_port, &to, &to_port), "the cut wire must be gone: {synapse_id}");
    }
    edit(&mut app, serde_json::json!([{ "operation": "connect", "sourceNodeId": from, "sourcePortId": from_port, "targetNodeId": to, "targetPortId": to_port }])).await;
    {
        let read = snapshot_read(&app);
        assert_eq!(read.host_snapshot.synapses.len(), before, "connect must restore exactly one wire");
        assert!(wire_exists(&read.host_snapshot, &from, &from_port, &to, &to_port), "connect must re-wire {from}:{from_port} -> {to}:{to_port}");
    }
    close(app);
}

//#region ✋️NodeDrag
/// ⚖️ LAW: a released node drag (the node-graph gesture record) is ONE edit, one row stamped with its transaction, whose
/// op is the relative `move-nodes` leaf; the dragged widget lands at its base position plus the offset.
#[semio_framework_async_macros::async_test]
async fn a_node_drag_record_is_one_transaction_of_one_relative_move() {
    let mut app = app().await;
    let (id, (x, y)) = place_every_widget(&mut app).await.remove(0);
    let ids = [id];
    let before = edit_rows(&mut app).await.len();
    edit(&mut app, serde_json::json!([{ "operation": "move", "gestureId": "node-drag:1", "nodeIds": [ids[0]], "dx": 40.0, "dy": -12.5 }])).await;
    assert_eq!(position(&app, &ids[0]), Some((x + 40.0, y - 12.5)));
    let rows = edit_rows(&mut app).await;
    let rows = &rows[before..];
    assert_eq!(rows.len(), 1, "one drag, one row: {rows:?}");
    let transaction = rows[0].transaction.as_ref().expect("the row is keyed by its tool transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool == "s.procedural.generation2d@1/*#editor#nodeGraphEdit", "{transaction:?}");
    assert!(rows[0].op_lines.iter().all(|line| line.starts_with("move-nodes")), "{:?}", rows[0].op_lines);
    assert_eq!(english(&rows[0]), "Move 1 node(s) by (40, -12.5)");
    assert_eq!(german(&rows[0]), "1 Knoten um (40; -12,5) verschieben");
    close(app);
}

/// ⚖️ LAW: a drag that moves nothing leaves zero trace; two drags are two transactions; a palette drop
/// (`moveMediaNode`) is the same relative leaf from the widget's base position.
#[semio_framework_async_macros::async_test]
async fn zero_trace_two_transactions_and_the_palette_drop() {
    let mut app = app().await;
    let (id, (x, y)) = place_every_widget(&mut app).await.remove(0);
    let ids = [id];
    let before = edit_rows(&mut app).await.len();
    edit(&mut app, serde_json::json!([{ "operation": "move", "gestureId": "node-drag:0", "nodeIds": [ids[0]], "dx": 0.0, "dy": 0.0 }])).await;
    edit(&mut app, serde_json::json!([{ "operation": "move", "gestureId": "node-drag:0", "nodeIds": ["ghost"], "dx": 10.0, "dy": 0.0 }])).await;
    assert_eq!(edit_rows(&mut app).await.len(), before, "nothing moved, nothing recorded");
    edit(&mut app, serde_json::json!([{ "operation": "move", "gestureId": "node-drag:1", "nodeIds": [ids[0]], "dx": 10.0, "dy": 0.0 }])).await;
    dispatch(&mut app, Generation2dCommand::MoveMediaNode(crate::editor::generation2d::commands::move_media_node::MoveMediaNode { node_id: ids[0].clone(), x: x + 30.0, y: y + 30.0 })).await;
    let rows = edit_rows(&mut app).await;
    let rows = &rows[before..];
    assert_eq!(rows.len(), 2, "{rows:?}");
    assert_ne!(rows[0].transaction.as_ref().expect("first").id, rows[1].transaction.as_ref().expect("second").id, "two moves are two transactions");
    assert_eq!(rows[1].transaction.as_ref().expect("drop").tool, "s.procedural.generation2d@1/*#editor#moveMediaNode");
    assert_eq!(english(&rows[1]), "Move 1 node(s) by (20, 30)", "the drop is the offset from the base position after the first drag");
    assert!(sits_at(&app, &ids[0], (x + 30.0, y + 30.0)), "{:?}", position(&app, &ids[0]));
    close(app);
}
//#endregion ✋️NodeDrag

//#region 🎚️SliderPress
/// ⚖️ LAW: a slider press keeps its ticks provisional (the committed value moves only at the release) and lands as ONE
/// edit with ONE transaction of the absolute leaf; a second press is a second transaction; a cancelled press leaves zero
/// trace.
#[semio_framework_async_macros::async_test]
async fn a_slider_press_is_one_transaction_and_a_cancel_is_zero_trace() {
    let mut app = app().await;
    let base = slider_value(&snapshot_read(&app).host_snapshot, "slider").expect("the demo slider");
    let before = edit_rows(&mut app).await.len();
    for (value, commit) in [(5.0, false), (6.0, false), (7.5, true)] {
        send(&mut app, serde_json::json!({ "operations": [{ "operation": "setSlider", "widgetId": "slider", "value": value }], "gesture": "press-1", "commit": commit })).await;
        assert_eq!(slider_value(&snapshot_read(&app).host_snapshot, "slider"), Some(if commit { value } else { base }), "a tick stays provisional");
    }
    send(&mut app, serde_json::json!({ "operations": [{ "operation": "setSlider", "widgetId": "slider", "value": 2.0 }], "gesture": "press-2", "commit": true })).await;
    send(&mut app, serde_json::json!({ "operations": [{ "operation": "setSlider", "widgetId": "slider", "value": 9.0 }], "gesture": "press-3", "commit": false })).await;
    send(&mut app, serde_json::json!({ "operations": [], "gesture": "press-3", "abort": "blur" })).await;
    assert_eq!(slider_value(&snapshot_read(&app).host_snapshot, "slider"), Some(2.0), "the cancelled press left no value");
    let rows = edit_rows(&mut app).await;
    let rows = &rows[before..];
    assert_eq!(rows.len(), 2, "two released presses, two rows: {rows:?}");
    assert!(rows.iter().all(|entry| entry.op_lines.iter().all(|line| line.starts_with("change-slider-value"))), "{rows:?}");
    let transactions: std::collections::BTreeSet<&str> = rows.iter().map(|entry| entry.transaction.as_ref().expect("every press is a transaction").id.as_str()).collect();
    assert_eq!(transactions.len(), 2);
    assert_eq!(english(&rows[0]), "Set slider \"slider\" to 7.5");
    assert_eq!(german(&rows[0]), "Schieberegler \"slider\" auf 7,5 setzen");
    close(app);
}

/// ⚖️ LAW: a live slider tick declares the narrow refresh scope.
#[test]
fn a_slider_tick_declares_the_narrow_scope() {
    match slider_gesture_ui_scope() {
        UiDirtyScope::Partial { utilities, tools, engagements, measures, labels, .. } => {
            assert_eq!((utilities, tools, engagements, measures, labels), (false, false, false, false, false));
        }
        other => panic!("a slider tick must declare a partial scope, got {other:?}"),
    }
}
//#endregion 🎚️SliderPress

//#region 🔗️EditRows
const NODE_GRAPH_EDIT_ROWS_JSON: &str = include_str!("../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json");

/// ⚖️ LAW (shared row contract, design §13.3; fixture `🧫️node-graph-edit-rows`): every accepted renderer row decodes, and
/// every refused one — a whole fixture (`setHostSnapshot`), an ambient-selection delete, an absolute move, an unknown
/// operation — refuses the whole `nodeGraphEdit` batch, both at admission and at authoring.
#[test]
fn node_graph_edit_takes_exactly_the_shared_row_vocabulary() {
    let fixture: serde_json::Value = serde_json::from_str(NODE_GRAPH_EDIT_ROWS_JSON).expect("node-graph edit rows fixture");
    for case in fixture["accepted"].as_array().expect("accepted rows") {
        assert!(rows(&NodeGraphEdit { operations_json: serde_json::json!([case["row"]]).to_string() }).is_ok(), "{} decodes", case["id"]);
    }
    for case in fixture["refused"].as_array().expect("refused rows") {
        let batch = serde_json::json!([{ "operation": "disconnect", "synapseId": "s1" }, case["row"]]);
        assert!(rows(&NodeGraphEdit { operations_json: batch.to_string() }).is_err(), "{} refuses the whole batch", case["id"]);
        let args: dsl::DslValue = serde_json::json!({ "operations": batch }).into();
        assert!(<crate::editor::generation2d::Generation2dPlayApp as semio_framework_plugin::ArtifactEditor>::command_from_action("nodeGraphEdit", Some(&args)).is_err(), "{} is refused at admission", case["id"]);
    }
}
//#endregion 🔗️EditRows

```

### ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs

SHA-256 80dcef207c8257c937c66743f1fe0b63ffc8bd0b835a41408e03971610b5e12c; 14375 bytes.

```rust
//! 🕸️ Laws of the `nodeGraphEdit` gesture leaves (design §5, §13.1, §13.3): a slider press is ONE tool transaction of
//! the ABSOLUTE `change-slider-value` leaf (ticks provisional, a cancel zero trace), a released node drag is ONE tool
//! transaction of the RELATIVE `move-nodes` leaf; each is one edit and one history row labelled from its leaf in English
//! and German, and two gestures are two transactions.

use super::*;
use crate::app_fixture::{app, dispatch, snapshot, Generation3dApp};
use crate::editor::generation3d::Generation3dCommand;
use semio_framework::kernel::HistoryEntry;
use semio_framework_plugin::artifact_app_laws::{meta, settle_registered_typed_operation};
use semio_framework_plugin::PluginApp;

const SLIDER_GESTURE_FIXTURE_JSON: &str = include_str!("../../../../../🧫️fixtures/🎚️slider-gesture.json");

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct SliderGestureFixture {
    schema: String,
    slider: String,
    presses: Vec<SliderPressRow>,
    ui_scope: SliderGestureUiScope,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct SliderPressRow {
    row: String,
    dispatches: Vec<serde_json::Value>,
    committed: Vec<Option<f64>>,
    edits: usize,
    transactions: usize,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct SliderGestureUiScope {
    window_bodies: Vec<String>,
    panel_bodies: Vec<String>,
    utilities: bool,
    tools: bool,
    engagements: bool,
    measures: bool,
    labels: bool,
}

fn slider_gesture_table() -> SliderGestureFixture {
    let table: SliderGestureFixture = serde_json::from_str(SLIDER_GESTURE_FIXTURE_JSON).expect("slider gesture fixture");
    assert_eq!(table.schema, "s.procedural.generation3d.slider-gesture/v2");
    table
}

fn slider_value(host_snapshot: &FlowHostSnapshot, widget_id: &str) -> Option<f64> {
    host_snapshot.widgets.iter().find_map(|widget| match widget {
        semio_framework_artifact_flow_flow::Widget::InputSlider { id, value, .. } if id == widget_id => Some(*value),
        _ => None,
    })
}

/// 🕹️ One `nodeGraphEdit` exactly as a host sends it (`operations` plus the press's top-level `gesture`/`commit`/`abort`),
/// settled through the registered ladder.
async fn send(app: &mut Generation3dApp, args: serde_json::Value) {
    let action_meta = meta("local");
    let args: dsl::DslValue = args.into();
    app.handle_action("nodeGraphEdit", Some(&args), &action_meta).await.expect("nodeGraphEdit admitted");
    settle_registered_typed_operation(app, action_meta.instance_id).await.expect("nodeGraphEdit settles");
}

/// 🧾️ Every applied history row that carries document operations, oldest first.
async fn edit_rows(app: &mut Generation3dApp) -> Vec<HistoryEntry> {
    let mut rows: Vec<HistoryEntry> = PluginApp::history_snapshot(app).await.expect("history").upserts.into_iter().filter(|entry| entry.applied && !entry.op_lines.is_empty()).collect();
    rows.sort_by_key(|entry| entry.seq);
    rows
}

/// 📍️ Lays every widget of the document out on the canvas (the `reorganize` verb, ONE setup edit), so a node drag has base
/// positions; answers the placed ids with their base positions.
async fn place_every_widget(app: &mut Generation3dApp) -> Vec<(String, (f64, f64))> {
    dispatch(app, Generation3dCommand::Reorganize(crate::editor::generation3d::commands::reorganize::Reorganize {})).await;
    let read = snapshot(app);
    read.host_snapshot.widgets.iter().map(|widget| crate::widget_id(widget).to_string()).filter_map(|id| read.host_snapshot.layout.get(id.as_str()).map(|layout| (layout.x, layout.y)).map(|base| (id, base))).collect()
}

fn english(entry: &HistoryEntry) -> String {
    entry.label.resolve(protocol::Terminology::Native, protocol::Locale::En).to_string()
}

fn german(entry: &HistoryEntry) -> String {
    entry.label.resolve(protocol::Terminology::Native, protocol::Locale::De).to_string()
}

//#region 🎚️SliderPress
/// ⚖️ LAW: every row of `🎚️slider-gesture.json` — the committed value moves only at the release, one press is one edit
/// stamped with one tool transaction whose op is the absolute `change-slider-value`, a cancelled press leaves zero trace.
#[semio_framework_async_macros::async_test]
async fn every_slider_press_row_is_one_transaction_on_the_released_value() {
    let _serial = crate::test_serial::lock();
    let table = slider_gesture_table();
    for row in &table.presses {
        assert_eq!(row.dispatches.len(), row.committed.len(), "row {} states one committed value per dispatch", row.row);
        let mut app = app().await;
        let base = slider_value(&snapshot(&app).host_snapshot, &table.slider).expect("the fixture slider exists");
        let before = edit_rows(&mut app).await.len();
        for (dispatch_row, committed) in row.dispatches.iter().zip(&row.committed) {
            let mut args = dispatch_row.clone();
            for operation in args["operations"].as_array_mut().into_iter().flatten() {
                if operation["value"] == "base" {
                    operation["value"] = serde_json::json!(base);
                }
            }
            send(&mut app, args).await;
            assert_eq!(slider_value(&snapshot(&app).host_snapshot, &table.slider), Some(committed.unwrap_or(base)), "row {} committed value after {dispatch_row}", row.row);
        }
        let rows = edit_rows(&mut app).await;
        let rows = &rows[before..];
        assert_eq!(rows.len(), row.edits, "row {} costs exactly {} edit(s): {rows:?}", row.row, row.edits);
        let transactions: std::collections::BTreeSet<&str> = rows.iter().filter_map(|entry| entry.transaction.as_ref()).map(|transaction| transaction.id.as_str()).collect();
        assert_eq!(transactions.len(), row.transactions, "row {} commits exactly {} transaction(s)", row.row, row.transactions);
        assert!(rows.iter().all(|entry| entry.op_lines.iter().all(|line| line.starts_with("change-slider-value"))), "row {}: every op is the absolute leaf: {rows:?}", row.row);
        assert!(rows.iter().filter_map(|entry| entry.transaction.as_ref()).all(|transaction| transaction.tool == "s.procedural.generation3d@1/*#editor#nodeGraphEdit"), "row {}: {rows:?}", row.row);
    }
}

/// ⚖️ LAW: a slider press row is labelled from its leaf in every locale.
#[semio_framework_async_macros::async_test]
async fn a_slider_press_row_is_labelled_from_its_leaf_in_english_and_german() {
    let _serial = crate::test_serial::lock();
    let mut app = app().await;
    send(&mut app, serde_json::json!({ "operations": [{ "operation": "setSlider", "widgetId": "height", "value": 7.25 }], "gesture": "g1", "commit": true })).await;
    let rows = edit_rows(&mut app).await;
    let row = rows.last().expect("the press is a row");
    assert_eq!(english(row), "Set slider \"height\" to 7.25");
    assert_eq!(german(row), "Schieberegler \"height\" auf 7,25 setzen");
}

/// ⚖️ LAW: a live slider tick declares the NARROW refresh scope the table states.
#[test]
fn a_slider_tick_declares_the_narrow_scope_the_table_states() {
    let table = slider_gesture_table();
    match slider_gesture_ui_scope() {
        UiDirtyScope::Partial { window_bodies, panel_bodies, utilities, tools, engagements, measures, labels } => {
            assert_eq!(window_bodies, table.ui_scope.window_bodies, "a slider tick repaints exactly the stated window bodies");
            assert_eq!(panel_bodies, table.ui_scope.panel_bodies, "a slider tick repaints exactly the stated panel bodies");
            assert_eq!((utilities, tools, engagements, measures, labels), (table.ui_scope.utilities, table.ui_scope.tools, table.ui_scope.engagements, table.ui_scope.measures, table.ui_scope.labels), "a slider tick touches no rail");
        }
        other => panic!("a slider tick must declare a partial scope, got {other:?}"),
    }
}
//#endregion 🎚️SliderPress

//#region ✋️NodeDrag
/// ⚖️ LAW: a released node drag (the node-graph gesture record) is ONE edit, one row stamped with its transaction, whose
/// op is the relative `move-nodes` leaf; every dragged widget lands at its base position plus the offset.
#[semio_framework_async_macros::async_test]
async fn a_node_drag_record_is_one_transaction_of_one_relative_move() {
    let _serial = crate::test_serial::lock();
    let mut app = app().await;
    let placed = place_every_widget(&mut app).await;
    let [(first, (first_x, first_y)), (second, (second_x, second_y))] = [placed[0].clone(), placed[1].clone()];
    let before = edit_rows(&mut app).await.len();
    send(&mut app, serde_json::json!({ "operations": [{ "operation": "move", "gestureId": "node-drag:1", "nodeIds": [first, second], "dx": 40.0, "dy": -12.5 }] })).await;
    let read = snapshot(&app);
    assert_eq!(read.host_snapshot.layout.get(first.as_str()).map(|layout| (layout.x, layout.y)), Some((first_x + 40.0, first_y - 12.5)));
    assert_eq!(read.host_snapshot.layout.get(second.as_str()).map(|layout| (layout.x, layout.y)), Some((second_x + 40.0, second_y - 12.5)));
    drop(read);
    let rows = edit_rows(&mut app).await;
    let rows = &rows[before..];
    assert_eq!(rows.len(), 1, "one drag, one row: {rows:?}");
    let transaction = rows[0].transaction.as_ref().expect("the row is keyed by its tool transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool == "s.procedural.generation3d@1/*#editor#nodeGraphEdit", "{transaction:?}");
    assert!(rows[0].op_lines.iter().all(|line| line.starts_with("move-nodes")), "{:?}", rows[0].op_lines);
    assert_eq!(english(&rows[0]), "Move 2 node(s) by (40, -12.5)");
    assert_eq!(german(&rows[0]), "2 Knoten um (40; -12,5) verschieben");
}

/// ⚖️ LAW: a drag that moves nothing — a zero offset or a stranger id — leaves zero trace; two drags are two
/// transactions.
#[semio_framework_async_macros::async_test]
async fn a_drag_that_moves_nothing_leaves_zero_trace_and_two_drags_are_two_transactions() {
    let _serial = crate::test_serial::lock();
    let mut app = app().await;
    let (id, (x, y)) = place_every_widget(&mut app).await.remove(0);
    let ids = [id];
    let before = edit_rows(&mut app).await.len();
    send(&mut app, serde_json::json!({ "operations": [{ "operation": "move", "gestureId": "node-drag:0", "nodeIds": [ids[0]], "dx": 0.0, "dy": 0.0 }] })).await;
    send(&mut app, serde_json::json!({ "operations": [{ "operation": "move", "gestureId": "node-drag:0", "nodeIds": ["ghost"], "dx": 10.0, "dy": 0.0 }] })).await;
    assert_eq!(edit_rows(&mut app).await.len(), before, "nothing moved, nothing recorded");
    send(&mut app, serde_json::json!({ "operations": [{ "operation": "move", "gestureId": "node-drag:1", "nodeIds": [ids[0]], "dx": 10.0, "dy": 0.0 }] })).await;
    send(&mut app, serde_json::json!({ "operations": [{ "operation": "move", "gestureId": "node-drag:2", "nodeIds": [ids[0]], "dx": 0.0, "dy": 10.0 }] })).await;
    let rows = edit_rows(&mut app).await;
    let rows = &rows[before..];
    assert_eq!(rows.len(), 2, "{rows:?}");
    assert_ne!(rows[0].transaction.as_ref().expect("first").id, rows[1].transaction.as_ref().expect("second").id, "two drags are two transactions");
    assert_eq!(snapshot(&app).host_snapshot.layout.get(ids[0].as_str()).map(|layout| (layout.x, layout.y)), Some((x + 10.0, y + 10.0)));
}
//#endregion ✋️NodeDrag

//#region 🔗️EditRows
const NODE_GRAPH_EDIT_ROWS_JSON: &str = include_str!("../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json");

/// ⚖️ LAW (shared row contract, design §13.3; fixture `🧫️node-graph-edit-rows`): every accepted renderer row decodes, and
/// every refused one — a whole fixture (`setHostSnapshot`), an ambient-selection delete, an absolute move, an unknown
/// operation — refuses the whole `nodeGraphEdit` batch, both at admission and at authoring, before anything is authored.
#[test]
fn node_graph_edit_takes_exactly_the_shared_row_vocabulary() {
    let fixture: serde_json::Value = serde_json::from_str(NODE_GRAPH_EDIT_ROWS_JSON).expect("node-graph edit rows fixture");
    for case in fixture["accepted"].as_array().expect("accepted rows") {
        assert!(rows(&NodeGraphEdit { operations_json: serde_json::json!([case["row"]]).to_string() }).is_ok(), "{} decodes", case["id"]);
    }
    for case in fixture["refused"].as_array().expect("refused rows") {
        let batch = serde_json::json!([{ "operation": "disconnect", "synapseId": "s1" }, case["row"]]);
        assert!(rows(&NodeGraphEdit { operations_json: batch.to_string() }).is_err(), "{} refuses the whole batch", case["id"]);
        let args: dsl::DslValue = serde_json::json!({ "operations": batch }).into();
        assert!(<crate::editor::generation3d::Generation3dPlayApp as semio_framework_plugin::ArtifactEditor>::command_from_action("nodeGraphEdit", Some(&args)).is_err(), "{} is refused at admission", case["id"]);
    }
}

/// ⚖️ LAW: a `delete {nodeIds, synapseIds}` row deletes exactly the widget and the wire it names, as ONE edit — no ambient
/// selection is read.
#[semio_framework_async_macros::async_test]
async fn a_delete_row_deletes_exactly_the_named_widget_and_wire_as_one_edit() {
    let _serial = crate::test_serial::lock();
    let mut app = app().await;
    let (wire, widget) = {
        let read = snapshot(&app);
        let wire = read.host_snapshot.synapses.first().expect("a wire").clone();
        let widget = read.host_snapshot.widgets.iter().map(crate::widget_id).find(|id| *id != wire.from && *id != wire.to).expect("a widget the wire does not hold").to_string();
        (wire.id, widget)
    };
    let before = edit_rows(&mut app).await.len();
    send(&mut app, serde_json::json!({ "operations": [{ "operation": "delete", "nodeIds": [widget], "synapseIds": [wire] }] })).await;
    let read = snapshot(&app);
    assert!(!read.host_snapshot.synapses.iter().any(|synapse| synapse.id == wire), "the named wire is gone");
    assert!(!read.host_snapshot.widgets.iter().any(|candidate| crate::widget_id(candidate) == widget), "the named widget is gone");
    drop(read);
    assert_eq!(edit_rows(&mut app).await.len() - before, 1, "one delete row, one edit");
}
//#endregion 🔗️EditRows

```

### 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json

SHA-256 74a67b08a92f953f617e2a0c065523486df7c9a443cbdced1f741f4db80c5d7e; 5425 bytes.

```json
{
  "what": "🔗️ The closed node-graph gesture record vocabulary every renderer dispatches as `nodeGraphEdit` arguments (design §13.3) and the add-node record of a flow canvas — accepted rows, refused rows, and catalogue descriptors with the `addWidget` arguments they become.",
  "why": "Ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING §12/§13.3: React published the WHOLE fixture (`setHostSnapshot`) for adds, aligns, port inserts and every generic graph gesture, so those edits reached history as one opaque snapshot leaf instead of intent leaves. Every host now dispatches the rows of one shared journal (`dag_graph_edit_rows_json`).",
  "implementations": {
    "schema": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧬️schema/🔣️node-graph-edit-rows/🔣️.json",
    "typescript": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧪️node-graph-edit-rows/🟦️.ts",
    "rustJournal": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🧪️node-graph-edit-rows/🦀️.rs",
    "rustFlowGuest": "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs"
  },
  "accepted": [
    { "id": "connect", "row": { "operation": "connect", "sourceNodeId": "slider", "sourcePortId": "number", "targetNodeId": "add", "targetPortId": "b" } },
    { "id": "disconnect", "row": { "operation": "disconnect", "synapseId": "s1" } },
    { "id": "move", "row": { "operation": "move", "gestureId": "node-drag:3", "nodeIds": ["add", "slider"], "dx": 284.5, "dy": -48 } },
    { "id": "set-slider", "row": { "operation": "setSlider", "widgetId": "slider", "value": 6.5 } },
    { "id": "insert-input-port", "row": { "operation": "insertPort", "nodeId": "add", "side": "input", "index": 2 } },
    { "id": "insert-output-port", "row": { "operation": "insertPort", "nodeId": "add", "side": "output", "index": 0 } },
    { "id": "delete-nodes-and-wires", "row": { "operation": "delete", "nodeIds": ["add"], "synapseIds": ["s1"] } },
    { "id": "delete-wires-only", "row": { "operation": "delete", "nodeIds": [], "synapseIds": ["s1", "s2"] } }
  ],
  "refused": [
    { "id": "whole-fixture", "row": { "operation": "setHostSnapshot", "hostSnapshotJson": "{}" } },
    { "id": "ambient-selection-delete", "row": { "operation": "deleteSelection" } },
    { "id": "absolute-move", "row": { "operation": "move", "nodeId": "add", "x": 568, "y": 96 } },
    { "id": "move-without-nodes", "row": { "operation": "move", "gestureId": "node-drag:3", "nodeIds": [], "dx": 1, "dy": 0 } },
    { "id": "move-repeated-node", "row": { "operation": "move", "gestureId": "node-drag:3", "nodeIds": ["add", "add"], "dx": 1, "dy": 0 } },
    { "id": "connect-extra-field", "row": { "operation": "connect", "sourceNodeId": "slider", "sourcePortId": "number", "targetNodeId": "add", "targetPortId": "b", "kind": "data" } },
    { "id": "disconnect-empty-id", "row": { "operation": "disconnect", "synapseId": "" } },
    { "id": "slider-text-value", "row": { "operation": "setSlider", "widgetId": "slider", "value": "6" } },
    { "id": "port-unknown-side", "row": { "operation": "insertPort", "nodeId": "add", "side": "left", "index": 1 } },
    { "id": "port-fractional-index", "row": { "operation": "insertPort", "nodeId": "add", "side": "input", "index": 1.5 } },
    { "id": "port-negative-index", "row": { "operation": "insertPort", "nodeId": "add", "side": "input", "index": -1 } },
    { "id": "delete-nothing", "row": { "operation": "delete", "nodeIds": [], "synapseIds": [] } },
    { "id": "delete-repeated-node", "row": { "operation": "delete", "nodeIds": ["add", "add"], "synapseIds": [] } },
    { "id": "unknown-operation", "row": { "operation": "align", "mode": "left" } }
  ],
  "answers": [
    { "id": "empty-journal", "json": "{\"operations\":[]}", "rows": [] },
    { "id": "not-json", "json": "not json", "rows": [] },
    { "id": "no-operations", "json": "{\"hostSnapshotChanged\":true}", "rows": [] },
    { "id": "rows-kept-junk-dropped", "json": "{\"operations\":[{\"operation\":\"disconnect\",\"synapseId\":\"s1\"},7,null,{\"synapseId\":\"s2\"}]}", "rows": [{ "operation": "disconnect", "synapseId": "s1" }] }
  ],
  "addWidget": [
    { "id": "operator", "descriptor": "{\"kind\":\"neuron\",\"neuronKind\":\"math.add\"}", "x": 40, "y": 41, "args": { "kind": "neuron", "neuronKind": "math.add", "x": 40, "y": 41 } },
    { "id": "labelled-slider", "descriptor": "{\"kind\":\"inputSlider\",\"label\":\"Height\"}", "x": 0, "y": -10, "args": { "kind": "inputSlider", "label": "Height", "x": 0, "y": -10 } },
    { "id": "export", "descriptor": "{\"kind\":\"outputExport\",\"format\":\"stl\"}", "x": 1, "y": 2, "args": { "kind": "outputExport", "format": "stl", "x": 1, "y": 2 } },
    { "id": "action", "descriptor": "{\"kind\":\"outputAction\",\"action\":\"flow.extension.reorganize\"}", "x": 3, "y": 4, "args": { "kind": "outputAction", "action": "flow.extension.reorganize", "x": 3, "y": 4 } },
    { "id": "non-text-fields-dropped", "descriptor": "{\"kind\":\"inputNote\",\"label\":7,\"id\":\"n1\"}", "x": 5, "y": 6, "args": { "kind": "inputNote", "x": 5, "y": 6 } }
  ]
}

```

### 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧬️schema/🔣️node-graph-edit-rows/🔣️.json

SHA-256 d073e757104a479ff4802c75f3d518ad21aabe5bf8e5bb5f7af8cf0635f220a1; 4515 bytes.

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "semio://os/renderer/engine/node-graph-edit-rows",
  "title": "NodeGraphEditRows",
  "description": "🔗️ The node-graph gesture records every renderer dispatches as the arguments of `nodeGraphEdit` (design §13.3): each row names its entities by id and carries an intent, never a whole fixture. `addWidget` is the add-node record a flow canvas dispatches as its own verb.",
  "type": "object",
  "additionalProperties": false,
  "required": ["operations"],
  "properties": {
    "operations": { "type": "array", "maxItems": 256, "items": { "$ref": "#/$defs/Row" } }
  },
  "$defs": {
    "Id": { "type": "string", "minLength": 1 },
    "Ids": { "type": "array", "uniqueItems": true, "items": { "$ref": "#/$defs/Id" } },
    "Row": {
      "oneOf": [
        { "$ref": "#/$defs/Connect" },
        { "$ref": "#/$defs/Disconnect" },
        { "$ref": "#/$defs/Move" },
        { "$ref": "#/$defs/SetSlider" },
        { "$ref": "#/$defs/InsertPort" },
        { "$ref": "#/$defs/Delete" }
      ]
    },
    "Connect": {
      "description": "🔌️ A wire drawn from an output port to an input port.",
      "type": "object",
      "additionalProperties": false,
      "required": ["operation", "sourceNodeId", "sourcePortId", "targetNodeId", "targetPortId"],
      "properties": {
        "operation": { "const": "connect" },
        "sourceNodeId": { "$ref": "#/$defs/Id" },
        "sourcePortId": { "type": "string" },
        "targetNodeId": { "$ref": "#/$defs/Id" },
        "targetPortId": { "type": "string" }
      }
    },
    "Disconnect": {
      "description": "✂️ A wire cut, by the synapse id the guest knows it as.",
      "type": "object",
      "additionalProperties": false,
      "required": ["operation", "synapseId"],
      "properties": { "operation": { "const": "disconnect" }, "synapseId": { "$ref": "#/$defs/Id" } }
    },
    "Move": {
      "description": "✋️ A released drag: the press, the moved node ids and their ONE relative offset.",
      "type": "object",
      "additionalProperties": false,
      "required": ["operation", "gestureId", "nodeIds", "dx", "dy"],
      "properties": {
        "operation": { "const": "move" },
        "gestureId": { "$ref": "#/$defs/Id" },
        "nodeIds": { "allOf": [{ "$ref": "#/$defs/Ids" }, { "type": "array", "minItems": 1 }] },
        "dx": { "type": "number" },
        "dy": { "type": "number" }
      }
    },
    "SetSlider": {
      "description": "🎚️ The value an inline slider was released on (its intent is the absolute value).",
      "type": "object",
      "additionalProperties": false,
      "required": ["operation", "widgetId", "value"],
      "properties": { "operation": { "const": "setSlider" }, "widgetId": { "$ref": "#/$defs/Id" }, "value": { "type": "number" } }
    },
    "InsertPort": {
      "description": "➕️ A variadic port inserted on a node at `index` of one side.",
      "type": "object",
      "additionalProperties": false,
      "required": ["operation", "nodeId", "side", "index"],
      "properties": {
        "operation": { "const": "insertPort" },
        "nodeId": { "$ref": "#/$defs/Id" },
        "side": { "enum": ["input", "output"] },
        "index": { "type": "integer", "minimum": 0, "maximum": 4294967295 }
      }
    },
    "Delete": {
      "description": "🗑️ The named nodes (with every wire they hold) and wires deleted.",
      "type": "object",
      "additionalProperties": false,
      "required": ["operation", "nodeIds", "synapseIds"],
      "properties": { "operation": { "const": "delete" }, "nodeIds": { "$ref": "#/$defs/Ids" }, "synapseIds": { "$ref": "#/$defs/Ids" } },
      "anyOf": [{ "properties": { "nodeIds": { "type": "array", "minItems": 1 } } }, { "properties": { "synapseIds": { "type": "array", "minItems": 1 } } }]
    },
    "AddWidget": {
      "description": "➕️ The add-node record of a flow canvas (the `addWidget` verb): the widget kind, the descriptor fields of its catalogue row and the world position; the owner mints the id.",
      "type": "object",
      "additionalProperties": false,
      "required": ["kind", "x", "y"],
      "properties": {
        "kind": { "$ref": "#/$defs/Id" },
        "neuronKind": { "$ref": "#/$defs/Id" },
        "label": { "type": "string" },
        "action": { "type": "string" },
        "format": { "type": "string" },
        "x": { "type": "number" },
        "y": { "type": "number" }
      }
    }
  }
}

```

### 🧰️framework/🔨️modules/🛠️tool-machine/🦀️.rs

SHA-256 c3ae8e2afe118cf7f2bd3ac5c238bbd1a9751c0d9dd2d21a4f9e3c59c55408c1; 70992 bytes.

```rust
//! 🛠️ Domain-neutral tool machines: a tool is a `🔄️machine` statechart whose effects are [`ToolYield`]s;
//! a [`ToolMachineRunner`] drives it through the machine kernel and folds the yields into at most one
//! open [`ToolTransaction`], which commits exactly one edit or aborts with zero trace. Tool state (the
//! statechart context and configuration) is never history-editable; only the committed mutations are.
//!
//! Pure and target-neutral: no async, no store, no clock of its own (every entry point that may run
//! machine actions takes the hybrid logical clock of that moment). The host can always cancel: `abort` and
//! `reset` drop the open transaction with zero trace and return the statechart to its initial configuration.
//! Schema of record: `🧬️schema/🔣️.json`.
//! Contract: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §5.

use machine::{Command, Configuration, Host, Machine, MachineDefinition, NullInspector, Snapshot, TimerId};
use protocol::{ActorId, HybridLogicalTimestamp, TransactionRef};

//#region 🔖️Yield
/// 🎇️ What a tool machine proposes to its transaction: keyed upserts and retractions of parametric
/// mutations, then exactly one closing `Commit` or `Abort`.
#[derive(Clone, Debug, PartialEq)]
pub enum ToolYield<M> {
    Upsert { key: String, mutation: M },
    Retract { key: String },
    Commit,
    Abort,
}

impl<M> ToolYield<M> {
    /// ➕️ Proposes `mutation` under `key`, replacing an earlier proposal with the same key in place.
    pub fn upsert(key: impl Into<String>, mutation: M) -> Self {
        Self::Upsert { key: key.into(), mutation }
    }

    /// ➖️ Withdraws the proposal under `key`, if any.
    pub fn retract(key: impl Into<String>) -> Self {
        Self::Retract { key: key.into() }
    }

    /// 🏷️ Wire tag of the variant.
    pub fn kind(&self) -> ToolYieldKind {
        match self {
            Self::Upsert { .. } => ToolYieldKind::Upsert,
            Self::Retract { .. } => ToolYieldKind::Retract,
            Self::Commit => ToolYieldKind::Commit,
            Self::Abort => ToolYieldKind::Abort,
        }
    }
}

/// 🗺️ Transaction-law matrix column of a yield.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ToolYieldKind {
    Upsert,
    Retract,
    Commit,
    Abort,
}

impl ToolYieldKind {
    pub const ALL: [Self; 4] = [Self::Upsert, Self::Retract, Self::Commit, Self::Abort];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Upsert => "upsert",
            Self::Retract => "retract",
            Self::Commit => "commit",
            Self::Abort => "abort",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.as_str() == text)
    }
}
//#endregion 🔖️Yield

//#region 🔖️Transaction
/// 🚦️ Lifecycle of one tool transaction; only `Open` accepts yields.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ToolTransactionState {
    Open,
    Committed,
    Aborted,
}

impl ToolTransactionState {
    pub const ALL: [Self; 3] = [Self::Open, Self::Committed, Self::Aborted];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Committed => "committed",
            Self::Aborted => "aborted",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|state| state.as_str() == text)
    }
}

/// 🔒️ Why a yield or an event was refused; every refusal leaves no trace. `Closed`: a yield reached a
/// transaction that already committed or aborted (or arrived while entering the initial configuration).
/// `Unclosed`: the tool came to rest with its transaction still open, which would let the next gesture join it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ToolRefusal {
    Closed,
    Unclosed,
}

impl ToolRefusal {
    pub const ALL: [Self; 2] = [Self::Closed, Self::Unclosed];

    /// 🔖️ Fault code, e.g. `toolTransaction.closed`.
    pub fn code(self) -> &'static str {
        match self {
            Self::Closed => "toolTransaction.closed",
            Self::Unclosed => "toolTransaction.unclosed",
        }
    }

    pub fn parse(code: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|refusal| refusal.code() == code)
    }
}

impl std::fmt::Display for ToolRefusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for ToolRefusal {}

/// 🛑️ Who ended a transaction without an edit: the tool itself (`Abort` yield) or the host (focus lost, pointer
/// capture lost, the gesture's base moved, a time-travel freeze, the tool retired).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ToolAbortReason {
    Tool,
    Blur,
    CaptureLost,
    BaseMoved,
    Frozen,
    Retired,
}

impl ToolAbortReason {
    pub const ALL: [Self; 6] = [Self::Tool, Self::Blur, Self::CaptureLost, Self::BaseMoved, Self::Frozen, Self::Retired];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Tool => "tool",
            Self::Blur => "blur",
            Self::CaptureLost => "captureLost",
            Self::BaseMoved => "baseMoved",
            Self::Frozen => "frozen",
            Self::Retired => "retired",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|reason| reason.as_str() == text)
    }
}

/// 🧾️ One interactive tool gesture or run: keyed provisional mutations in first-insertion order that
/// commit as ONE edit (one undo step, one history row) or abort leaving nothing behind.
#[derive(Clone, Debug, PartialEq)]
pub struct ToolTransaction<M> {
    reference: TransactionRef,
    state: ToolTransactionState,
    entries: Vec<(String, M)>,
}

impl<M> ToolTransaction<M> {
    /// 🌱️ An empty open transaction.
    pub fn open(reference: TransactionRef) -> Self {
        Self { reference, state: ToolTransactionState::Open, entries: Vec::new() }
    }

    /// ↩️ An open transaction restored from persisted tool state: `entries` are upserted in order, so their
    /// first-insertion order is kept and a repeated key keeps its first slot with its last mutation.
    pub fn resume(reference: TransactionRef, entries: Vec<(String, M)>) -> Self {
        let mut transaction = Self::open(reference);
        for (key, mutation) in entries {
            transaction.upsert(key, mutation);
        }
        transaction
    }

    fn upsert(&mut self, key: String, mutation: M) {
        match self.entries.iter_mut().find(|(existing, _)| *existing == key) {
            Some(entry) => entry.1 = mutation,
            None => self.entries.push((key, mutation)),
        }
    }

    /// 🪪️ The ref every op of the committed edit is stamped with.
    pub fn reference(&self) -> &TransactionRef {
        &self.reference
    }

    pub fn state(&self) -> ToolTransactionState {
        self.state
    }

    /// 📋️ Provisional `(key, mutation)` entries in first-insertion order (the preview overlay).
    pub fn entries(&self) -> &[(String, M)] {
        &self.entries
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// ⚖️ The reducer: upsert replaces by key keeping the first-insertion slot, retract removes, commit
    /// closes keeping the entries, abort closes discarding them; every yield on a closed transaction is refused.
    pub fn apply(&mut self, yielded: ToolYield<M>) -> Result<(), ToolRefusal> {
        if self.state != ToolTransactionState::Open {
            return Err(ToolRefusal::Closed);
        }
        match yielded {
            ToolYield::Upsert { key, mutation } => self.upsert(key, mutation),
            ToolYield::Retract { key } => self.entries.retain(|(existing, _)| *existing != key),
            ToolYield::Commit => self.state = ToolTransactionState::Committed,
            ToolYield::Abort => {
                self.state = ToolTransactionState::Aborted;
                self.entries.clear();
            }
        }
        Ok(())
    }

    /// 📦️ The committed batch in entry order, consuming the transaction.
    pub fn into_parts(self) -> (TransactionRef, Vec<M>) {
        (self.reference, self.entries.into_iter().map(|(_, mutation)| mutation).collect())
    }
}
//#endregion 🔖️Transaction

//#region 🔖️Machine
/// 🎰️ A `🔄️machine` statechart whose effects are [`ToolYield`]s over its `Mutation`. Blanket-implemented:
/// every `statechart!` declaring `effect: ToolYield<M>;` is a tool machine over `M`.
pub trait ToolMachine: Machine<Effect = ToolYield<<Self as ToolMachine>::Mutation>> {
    type Mutation;
}

impl<T: Machine<Effect = ToolYield<M>>, M> ToolMachine for T {
    type Mutation = M;
}

/// 🧷️ The single kernel actor a runner drives.
pub const TOOL_MACHINE_ACTOR: machine::ActorId = machine::ActorId(0);

/// 🔁️ What one event did to the runner's transaction slot.
#[derive(Clone, Debug, PartialEq)]
pub enum ToolStep<M> {
    /// 💤️ No transaction is open and none closed.
    Idle,
    /// ✏️ A transaction is open; its provisional entries are [`ToolMachineRunner::transaction`].
    Open,
    /// 💾️ A non-empty transaction committed: publish exactly one edit stamped with the ref.
    Committed(TransactionRef, Vec<M>),
    /// 🗑️ The transaction aborted, by the tool or the host: nothing is published and nothing remains.
    Aborted(TransactionRef, ToolAbortReason),
    /// 🫙️ The transaction committed empty: no edit.
    Empty(TransactionRef),
}

impl<M> ToolStep<M> {
    /// 🔤️ Wire tag of the step.
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Open => "open",
            Self::Committed(..) => "committed",
            Self::Aborted(..) => "aborted",
            Self::Empty(_) => "empty",
        }
    }
}

/// 🏃️ Drives one tool machine through the kernel and owns at most one open transaction. A transaction
/// opens at the first `Upsert` while none is open, its id minted from `(actor, clock, tool)` of that
/// event; `Retract`, `Commit` and `Abort` without an open transaction change nothing. Every event is
/// fail-closed: a yield after the close within the same event is refused (`Closed`), and so is an event that
/// leaves the tool at rest (its root's initial state active) with a transaction still open (`Unclosed`); a
/// refused event publishes nothing and drops the transaction. Non-yield commands (timers, invokes) go to the host.
pub struct ToolMachineRunner<T: ToolMachine, H: Host<T>> {
    pub host: H,
    tool: String,
    actor: ActorId,
    input: T::Input,
    snapshot: Snapshot<T>,
    transaction: Option<ToolTransaction<T::Mutation>>,
}

impl<T: ToolMachine, H: Host<T>> ToolMachineRunner<T, H>
where
    T::Input: Clone,
{
    /// 🚀️ Enters the initial configuration. A transaction only opens in response to an event, so a yield
    /// while entering is refused.
    pub fn start(tool: impl Into<String>, actor: ActorId, input: T::Input, mut host: H) -> Result<Self, ToolRefusal> {
        let mut commands = Vec::new();
        let mut snapshot = machine::init::<T>(input.clone(), &mut commands);
        let mut refused = false;
        for command in commands {
            match command {
                Command::Effect(_) => refused = true,
                other => {
                    machine::route_command(&mut host, &mut snapshot, TOOL_MACHINE_ACTOR, other);
                }
            }
        }
        if refused {
            return Err(ToolRefusal::Closed);
        }
        Ok(Self { host, tool: tool.into(), actor, input, snapshot, transaction: None })
    }

    /// ⏯️ Rebuilds a runner from persisted tool state ([`Self::into_parts`]) to continue its gesture: refuses a
    /// committed or aborted transaction (`Closed`) and a resting snapshot that holds an open transaction
    /// (`Unclosed`). Timers scheduled before the persist stay with the host that scheduled them.
    pub fn resume(tool: impl Into<String>, actor: ActorId, input: T::Input, snapshot: Snapshot<T>, transaction: Option<ToolTransaction<T::Mutation>>, host: H) -> Result<Self, ToolRefusal> {
        let runner = Self { host, tool: tool.into(), actor, input, snapshot, transaction };
        match &runner.transaction {
            Some(transaction) if transaction.state() != ToolTransactionState::Open => Err(ToolRefusal::Closed),
            Some(_) if runner.at_rest() => Err(ToolRefusal::Unclosed),
            _ => Ok(runner),
        }
    }

    /// 🧳️ The tool state to persist between dispatches (window transient): the statechart snapshot and the
    /// open transaction; [`Self::resume`] continues from them.
    pub fn into_parts(self) -> (Snapshot<T>, Option<ToolTransaction<T::Mutation>>) {
        (self.snapshot, self.transaction)
    }

    /// 🔧️ The authoring tool id `<appId>#<toolId>` stamped into every ref.
    pub fn tool(&self) -> &str {
        &self.tool
    }

    pub fn actor(&self) -> &ActorId {
        &self.actor
    }

    /// 📸️ The tool state: configuration and context (ephemeral, never history).
    pub fn snapshot(&self) -> &Snapshot<T> {
        &self.snapshot
    }

    /// 📝️ The open transaction, if any.
    pub fn transaction(&self) -> Option<&ToolTransaction<T::Mutation>> {
        self.transaction.as_ref()
    }

    /// 🛋️ Whether the tool rests: the root's initial state is active.
    pub fn at_rest(&self) -> bool {
        T::definition().nodes[machine::ROOT.0 as usize].initial.is_some_and(|initial| self.snapshot.configuration.contains(initial))
    }

    /// 🧯️ Host cancel (focus or capture lost, base moved, freeze, retirement): drops the open transaction with
    /// zero trace and returns the statechart to its initial configuration; `Aborted(ref, reason)`, or `Idle`
    /// when no transaction was open.
    pub fn abort(&mut self, reason: ToolAbortReason) -> ToolStep<T::Mutation> {
        self.rest().map_or(ToolStep::Idle, |reference| ToolStep::Aborted(reference, reason))
    }

    /// ♻️ Silent host reset to the freshly started runner (same input): zero trace, initial configuration;
    /// answers the ref of the transaction it dropped, if one was open.
    pub fn reset(&mut self) -> Option<TransactionRef> {
        self.rest()
    }

    fn rest(&mut self) -> Option<TransactionRef> {
        let definition = T::definition();
        for id in self.snapshot.configuration.iter_ones() {
            let node = &definition.nodes[id.0 as usize];
            for (timer, _) in node.timers {
                self.host.cancel_timer(TOOL_MACHINE_ACTOR, *timer);
            }
            for invoke in node.invokes {
                self.host.cancel_task(TOOL_MACHINE_ACTOR, *invoke);
            }
        }
        let mut commands = Vec::new();
        self.snapshot = machine::init::<T>(self.input.clone(), &mut commands);
        for command in commands {
            if !matches!(command, Command::Effect(_)) {
                machine::route_command(&mut self.host, &mut self.snapshot, TOOL_MACHINE_ACTOR, command);
            }
        }
        self.transaction.take().map(|transaction| transaction.reference)
    }

    /// 📨️ Runs `event` to completion and settles its yields.
    pub fn send(&mut self, event: T::Event, clock: HybridLogicalTimestamp) -> Result<ToolStep<T::Mutation>, ToolRefusal> {
        let mut commands = Vec::new();
        machine::macrostep(&mut self.snapshot, event, &mut commands, &mut NullInspector);
        self.settle(commands, clock)
    }

    /// ⏱️ Runs an elapsed `after` timer to completion and settles its yields.
    pub fn timer_elapsed(&mut self, timer: TimerId, clock: HybridLogicalTimestamp) -> Result<ToolStep<T::Mutation>, ToolRefusal> {
        let mut commands = Vec::new();
        machine::timer_elapsed(&mut self.snapshot, timer, &mut commands, &mut NullInspector);
        self.settle(commands, clock)
    }

    fn settle(&mut self, commands: Vec<Command<T>>, clock: HybridLogicalTimestamp) -> Result<ToolStep<T::Mutation>, ToolRefusal> {
        let mut refused = false;
        for command in commands {
            let Command::Effect(yielded) = command else {
                machine::route_command(&mut self.host, &mut self.snapshot, TOOL_MACHINE_ACTOR, command);
                continue;
            };
            if refused {
                continue;
            }
            match (&mut self.transaction, yielded) {
                (Some(transaction), yielded) => refused = transaction.apply(yielded).is_err(),
                (slot @ None, yielded @ ToolYield::Upsert { .. }) => {
                    let mut transaction = ToolTransaction::open(TransactionRef::mint(&self.actor, &clock, self.tool.as_str()));
                    refused = transaction.apply(yielded).is_err();
                    *slot = Some(transaction);
                }
                (None, _) => {}
            }
        }
        if refused {
            self.transaction = None;
            return Err(ToolRefusal::Closed);
        }
        if self.transaction.as_ref().is_some_and(|transaction| transaction.state() == ToolTransactionState::Open) && self.at_rest() {
            self.transaction = None;
            return Err(ToolRefusal::Unclosed);
        }
        Ok(match self.transaction.take() {
            None => ToolStep::Idle,
            Some(transaction) => match transaction.state() {
                ToolTransactionState::Open => {
                    self.transaction = Some(transaction);
                    ToolStep::Open
                }
                ToolTransactionState::Aborted => ToolStep::Aborted(transaction.reference, ToolAbortReason::Tool),
                ToolTransactionState::Committed if transaction.is_empty() => ToolStep::Empty(transaction.reference),
                ToolTransactionState::Committed => {
                    let (reference, mutations) = transaction.into_parts();
                    ToolStep::Committed(reference, mutations)
                }
            },
        })
    }
}
//#endregion 🔖️Machine

//#region 🔖️Scrub
/// 🎚️ The argument naming the press a continuous control's dispatch belongs to (`"<control>:<ms>"`); a dispatch
/// without it is a plain one-shot edit.
pub const SCRUB_GESTURE_ARG: &str = "gesture";
/// 🏁️ The argument marking the release (`true`) of a press.
pub const SCRUB_COMMIT_ARG: &str = "commit";
/// 🧯️ The argument carrying a host cancel's [`ToolAbortReason`] (`blur`, `captureLost`, `frozen`, `baseMoved`,
/// `retired`); the dispatch carries no value.
pub const SCRUB_ABORT_ARG: &str = "abort";

/// 🎚️ Where one dispatch of a continuous control (slider, held spinner, number field) sits in its press.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScrubPhase {
    Tick { gesture: String },
    Commit { gesture: String },
    Abort { gesture: String, reason: ToolAbortReason },
}

impl ScrubPhase {
    /// 🧩️ Reads the scrub arguments: `None` without a non-empty `gesture` (a one-shot dispatch) or with an unknown
    /// abort reason; `abort` wins over `commit`.
    pub fn parse(gesture: Option<&str>, commit: Option<bool>, abort: Option<&str>) -> Option<Self> {
        let gesture = gesture.filter(|gesture| !gesture.is_empty())?.to_string();
        match abort {
            Some(reason) => ToolAbortReason::parse(reason).map(|reason| Self::Abort { gesture, reason }),
            None if commit == Some(true) => Some(Self::Commit { gesture }),
            None => Some(Self::Tick { gesture }),
        }
    }

    /// 🆔️ The press this dispatch belongs to.
    pub fn gesture(&self) -> &str {
        match self {
            Self::Tick { gesture } | Self::Commit { gesture } | Self::Abort { gesture, .. } => gesture,
        }
    }

    /// 🧮️ The scrub input of this phase once the plugin's leaf constructor produced the ABSOLUTE `leaves` of its value
    /// (`set-x{target, value}`); an abort carries none.
    pub fn input<M>(self, leaves: Vec<M>) -> ScrubInput<M> {
        match self {
            Self::Tick { gesture } => ScrubInput::Tick { gesture, leaves },
            Self::Commit { gesture } => ScrubInput::Commit { gesture, leaves },
            Self::Abort { reason, .. } => ScrubInput::Abort { reason },
        }
    }
}

/// 📨️ What reaches a scrub: a live value's absolute leaves, the release's leaves, or a host cancel.
#[derive(Clone, Debug, PartialEq)]
pub enum ScrubInput<M> {
    Tick { gesture: String, leaves: Vec<M> },
    Commit { gesture: String, leaves: Vec<M> },
    Abort { reason: ToolAbortReason },
}

/// 🧰️ A scrub's tool state: the press it follows and how many keyed leaves (`"0"`, `"1"`, …) its transaction holds.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ScrubContext {
    pub gesture: Option<String>,
    pub keys: usize,
}

/// 📨️ The scrub statechart's events; the host cancel is the runner's [`ToolMachineRunner::abort`], never an event.
#[derive(Clone, Debug, PartialEq)]
pub enum ScrubEvent<M> {
    Tick { gesture: String, leaves: Vec<M> },
    Commit { gesture: String, leaves: Vec<M> },
}

impl<M: Clone> machine::StatechartEvent for ScrubEvent<M> {
    const EVENT_COUNT: u16 = 2;

    fn event_id(&self) -> machine::EventId {
        match self {
            Self::Tick { .. } => machine::EventId(0),
            Self::Commit { .. } => machine::EventId(1),
        }
    }

    fn event_name(id: machine::EventId) -> &'static str {
        match id.0 {
            0 => "Tick",
            1 => "Commit",
            _ => "?",
        }
    }
}

/// 🎚️ The ONE continuous-control tool: `idle → scrubbing` on a `Tick` (the press opens), `scrubbing → scrubbing` on a
/// `Tick` of the same press, `Commit` of the same press back to `idle` (a `Commit` from `idle` is a one-shot press).
/// Every tick replaces the transaction's entries with the tick's absolute leaves (upsert by position, retract the
/// rest), so the transaction always holds the net value; the release commits ONE edit, a host abort leaves zero
/// trace. The plugin supplies only its leaf constructor. Tables are M-independent and pinned against the
/// `statechart!` compilation of the same chart by the unit laws.
pub struct ScrubMachine<M>(std::marker::PhantomData<fn() -> M>);

const SCRUB_NODES: [machine::NodeDef; 3] = [
    machine::NodeDef { stable_id: "root", kind: machine::NodeKind::Compound, parent: None, initial: Some(machine::NodeId(1)), children: &[machine::NodeId(1), machine::NodeId(2)], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 0 },
    machine::NodeDef { stable_id: "idle", kind: machine::NodeKind::Atomic, parent: Some(machine::NodeId(0)), initial: None, children: &[], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 1 },
    machine::NodeDef { stable_id: "scrubbing", kind: machine::NodeKind::Atomic, parent: Some(machine::NodeId(0)), initial: None, children: &[], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 2 },
];

const SCRUB_TRANSITIONS: [machine::TransitionDef; 4] = [
    machine::TransitionDef { source: machine::NodeId(1), trigger: machine::Trigger::Event(machine::EventId(0)), guard: None, targets: &[machine::NodeId(2)], kind: machine::TransitionKind::External, actions: &[machine::ActionId(0)], doc_index: 0 },
    machine::TransitionDef { source: machine::NodeId(1), trigger: machine::Trigger::Event(machine::EventId(1)), guard: None, targets: &[machine::NodeId(1)], kind: machine::TransitionKind::External, actions: &[machine::ActionId(1)], doc_index: 1 },
    machine::TransitionDef { source: machine::NodeId(2), trigger: machine::Trigger::Event(machine::EventId(0)), guard: Some(machine::GuardId(0)), targets: &[machine::NodeId(2)], kind: machine::TransitionKind::External, actions: &[machine::ActionId(0)], doc_index: 2 },
    machine::TransitionDef { source: machine::NodeId(2), trigger: machine::Trigger::Event(machine::EventId(1)), guard: Some(machine::GuardId(0)), targets: &[machine::NodeId(1)], kind: machine::TransitionKind::External, actions: &[machine::ActionId(1)], doc_index: 3 },
];

/// 🔏️ The `statechart!` fingerprint of the scrub chart (restore gate of a persisted scrub).
pub const SCRUB_FINGERPRINT: u64 = 16240238296638685209;
/// 🗺️ The `statechart!` manifest of the scrub chart.
pub const SCRUB_MANIFEST_JSON: &str = r#"{"id":"scrub","states":[{"id":"root","parent":null},{"id":"idle","parent":0},{"id":"scrubbing","parent":0}],"events":["Tick","Commit"],"transitionCount":4}"#;

impl<M: Clone + 'static> ScrubMachine<M> {
    const DEFINITION: MachineDefinition<Self> = MachineDefinition {
        id: "scrub",
        nodes: &SCRUB_NODES,
        transitions: &SCRUB_TRANSITIONS,
        context_from_input: scrub_context,
        make_output: None,
        guards: &[scrub_same_gesture::<M>],
        actions: &[scrub_follow::<M>, scrub_settle::<M>],
        fingerprint: SCRUB_FINGERPRINT,
        manifest_json: SCRUB_MANIFEST_JSON,
    };
}

impl<M: Clone + 'static> Machine for ScrubMachine<M> {
    type Context = ScrubContext;
    type Event = ScrubEvent<M>;
    type Input = ScrubContext;
    type Output = ();
    type Effect = ToolYield<M>;
    type Config = machine::BitSet<1>;

    fn definition() -> &'static MachineDefinition<Self> {
        &Self::DEFINITION
    }
}

fn scrub_context(input: ScrubContext) -> ScrubContext {
    input
}

fn scrub_same_gesture<M>(context: &ScrubContext, event: Option<&ScrubEvent<M>>) -> bool {
    matches!(event, Some(ScrubEvent::Tick { gesture, .. } | ScrubEvent::Commit { gesture, .. }) if context.gesture.as_deref() == Some(gesture.as_str()))
}

fn scrub_follow<M: Clone + 'static>(context: &mut ScrubContext, event: Option<&ScrubEvent<M>>, sink: &mut Vec<Command<ScrubMachine<M>>>) {
    let Some(ScrubEvent::Tick { gesture, leaves }) = event else { return };
    context.gesture = Some(gesture.clone());
    scrub_replace(context, leaves, sink);
}

fn scrub_settle<M: Clone + 'static>(context: &mut ScrubContext, event: Option<&ScrubEvent<M>>, sink: &mut Vec<Command<ScrubMachine<M>>>) {
    let Some(ScrubEvent::Commit { leaves, .. }) = event else { return };
    scrub_replace(context, leaves, sink);
    sink.push(Command::Effect(ToolYield::Commit));
    *context = ScrubContext::default();
}

fn scrub_replace<M: Clone + 'static>(context: &mut ScrubContext, leaves: &[M], sink: &mut Vec<Command<ScrubMachine<M>>>) {
    sink.extend(leaves.iter().enumerate().map(|(index, leaf)| Command::Effect(ToolYield::upsert(index.to_string(), leaf.clone()))));
    sink.extend((leaves.len()..context.keys).map(|index| Command::Effect(ToolYield::retract(index.to_string()))));
    context.keys = leaves.len();
}

/// 🧷️ The scrub's host: the chart declares no timer, no invoke and no foreign effect, so every duty is empty.
pub struct ScrubHost;

impl<M: Clone + 'static> Host<ScrubMachine<M>> for ScrubHost {
    fn execute_effect(&mut self, _actor: machine::ActorId, _effect: ToolYield<M>) {}
    fn schedule(&mut self, _actor: machine::ActorId, _timer: TimerId, _delay_ms: u64) {}
    fn cancel_timer(&mut self, _actor: machine::ActorId, _timer: TimerId) {}
    fn start_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn cancel_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn now_ms(&self) -> u64 {
        0
    }
}

/// 💾️ One window's open scrub between dispatches (window transient, ephemeral local-only, never history): the
/// configuration by stable ids, the authoring tool `<appId>#<verb>` and actor, the press, the document revision it
/// opened on, and the open transaction with its keyed leaves.
#[derive(Clone, Debug, PartialEq)]
pub struct ScrubState<M> {
    pub states: Vec<String>,
    pub tool: String,
    pub actor: String,
    pub gesture: String,
    pub base_revision: String,
    pub transaction: TransactionRef,
    pub entries: Vec<(String, M)>,
}

/// 🎚️ One press of a continuous control: [`ScrubMachine`] under a [`ToolMachineRunner`], opened on one document
/// revision. A tick or release of ANOTHER press first host-aborts the open one (`captureLost`).
pub struct Scrub<M: Clone + 'static> {
    runner: ToolMachineRunner<ScrubMachine<M>, ScrubHost>,
    base_revision: String,
}

impl<M: Clone + 'static> Scrub<M> {
    /// 🚀️ A scrub at rest for `tool` (`<appId>#<verb>`) by `actor` on the document revision `base_revision`.
    pub fn start(tool: impl Into<String>, actor: ActorId, base_revision: impl Into<String>) -> Self {
        let runner = ToolMachineRunner::start(tool, actor, ScrubContext::default(), ScrubHost).expect("the scrub chart yields nothing while entering");
        Self { runner, base_revision: base_revision.into() }
    }

    /// ⏯️ The scrub a window persisted, restored by stable ids with its open transaction; a state the chart cannot
    /// restore is refused (`Closed`), so the caller drops it with zero trace.
    pub fn resume(state: ScrubState<M>) -> Result<Self, ToolRefusal> {
        let persisted = machine::PersistedSnapshot { version: 1, fingerprint: SCRUB_FINGERPRINT, states: state.states, history: Vec::new(), done: false };
        let context = ScrubContext { gesture: Some(state.gesture), keys: state.entries.len() };
        let snapshot = machine::restore::<ScrubMachine<M>, machine::NoMigrations>(&persisted, context, &[]).map_err(|_| ToolRefusal::Closed)?;
        let transaction = ToolTransaction::resume(state.transaction, state.entries);
        let runner = ToolMachineRunner::resume(state.tool, ActorId(state.actor), ScrubContext::default(), snapshot, Some(transaction), ScrubHost)?;
        Ok(Self { runner, base_revision: state.base_revision })
    }

    /// 🆔️ The press the scrub follows, while one is open.
    pub fn gesture(&self) -> Option<&str> {
        self.runner.snapshot().context.gesture.as_deref()
    }

    /// 📐️ The document revision the scrub opened on.
    pub fn base_revision(&self) -> &str {
        &self.base_revision
    }

    /// 🔧️ The authoring tool id.
    pub fn tool(&self) -> &str {
        self.runner.tool()
    }

    /// 📝️ The open transaction: the provisional leaves the preview overlays.
    pub fn transaction(&self) -> Option<&ToolTransaction<M>> {
        self.runner.transaction()
    }

    /// 📨️ Runs one input on `clock`: a host abort drops the open transaction with zero trace; a tick or release of
    /// another press first host-aborts the open one (`captureLost`).
    pub fn send(&mut self, input: ScrubInput<M>, clock: HybridLogicalTimestamp) -> Result<ToolStep<M>, ToolRefusal> {
        let event = match input {
            ScrubInput::Abort { reason } => return Ok(self.runner.abort(reason)),
            ScrubInput::Tick { gesture, leaves } => ScrubEvent::Tick { gesture, leaves },
            ScrubInput::Commit { gesture, leaves } => ScrubEvent::Commit { gesture, leaves },
        };
        let (ScrubEvent::Tick { gesture, .. } | ScrubEvent::Commit { gesture, .. }) = &event;
        if self.gesture().is_some_and(|open| open != gesture) {
            self.runner.abort(ToolAbortReason::CaptureLost);
        }
        self.runner.send(event, clock)
    }

    /// 💾️ The state to persist: `Some` only while a transaction is open.
    pub fn persist(self) -> Option<ScrubState<M>> {
        let (tool, actor) = (self.runner.tool().to_string(), self.runner.actor().0.clone());
        let (snapshot, transaction) = self.runner.into_parts();
        let transaction = transaction.filter(|transaction| transaction.state() == ToolTransactionState::Open)?;
        let gesture = snapshot.context.gesture.clone()?;
        Some(ScrubState { states: machine::persist(&snapshot).states, tool, actor, gesture, base_revision: self.base_revision, transaction: transaction.reference().clone(), entries: transaction.entries().to_vec() })
    }
}

/// 🗂️ Every window's open scrub (at most one per window) plus the press each window last closed, so a late tick of
/// a settled or cancelled press leaves zero trace. Pure: the runtime keeps one per app instance and overlays
/// [`Self::provisional`] on the committed document for every render.
#[derive(Clone, Debug, PartialEq)]
pub struct ScrubLedger<M> {
    windows: std::collections::BTreeMap<String, ScrubState<M>>,
    closed: std::collections::BTreeMap<String, String>,
}

impl<M> Default for ScrubLedger<M> {
    fn default() -> Self {
        Self { windows: std::collections::BTreeMap::new(), closed: std::collections::BTreeMap::new() }
    }
}

impl<M: Clone + 'static> ScrubLedger<M> {
    /// 🛋️ Whether no window holds an open scrub.
    pub fn is_empty(&self) -> bool {
        self.windows.is_empty()
    }

    /// 🔎️ The open scrub of `window`.
    pub fn open(&self, window: &str) -> Option<&ScrubState<M>> {
        self.windows.get(window)
    }

    /// 🪟️ The windows holding an open scrub, in window id order.
    pub fn windows(&self) -> impl Iterator<Item = &str> {
        self.windows.keys().map(String::as_str)
    }

    /// 👁️ Every open scrub's provisional leaves, window by window in window id order — the overlay a render applies
    /// on the committed document; never history.
    pub fn provisional(&self) -> impl Iterator<Item = &M> {
        self.windows.values().flat_map(|state| state.entries.iter().map(|(_, leaf)| leaf))
    }

    /// 📨️ Runs one input of `window`'s press. A tick or release of the press the window last closed is a silent no-op;
    /// an open scrub of another tool or opened on another document revision is host-aborted first (`captureLost`,
    /// `baseMoved`, zero trace), so the input opens a fresh transaction on the current revision — the leaves are
    /// absolute. The release closes the press.
    pub fn send(&mut self, window: &str, tool: &str, actor: &ActorId, base_revision: &str, input: ScrubInput<M>, clock: HybridLogicalTimestamp) -> Result<ToolStep<M>, ToolRefusal> {
        let (gesture, release) = match &input {
            ScrubInput::Abort { reason } => return Ok(self.abort(window, None, *reason)),
            ScrubInput::Tick { gesture, .. } => (gesture.clone(), false),
            ScrubInput::Commit { gesture, .. } => (gesture.clone(), true),
        };
        if self.closed.get(window) == Some(&gesture) {
            return Ok(ToolStep::Idle);
        }
        let open = self.windows.remove(window).filter(|state| state.tool == tool && state.base_revision == base_revision);
        let mut scrub = match open.map(Scrub::resume) {
            Some(Ok(scrub)) => scrub,
            _ => Scrub::start(tool, actor.clone(), base_revision),
        };
        let step = scrub.send(input, clock);
        if release {
            self.closed.insert(window.to_string(), gesture);
        }
        if let Some(state) = scrub.persist() {
            self.windows.insert(window.to_string(), state);
        }
        step
    }

    /// 🧯️ Host cancel of `window`'s open scrub (only of the press `gesture` when named): zero trace, and the press is
    /// closed so its late ticks stay silent. `Aborted(ref, reason)`, or `Idle` when no such scrub was open.
    pub fn abort(&mut self, window: &str, gesture: Option<&str>, reason: ToolAbortReason) -> ToolStep<M> {
        if let Some(gesture) = gesture {
            self.closed.insert(window.to_string(), gesture.to_string());
        }
        match self.windows.remove(window) {
            Some(state) if gesture.is_none_or(|gesture| gesture == state.gesture) => {
                self.closed.insert(window.to_string(), state.gesture);
                ToolStep::Aborted(state.transaction, reason)
            }
            Some(state) => {
                self.windows.insert(window.to_string(), state);
                ToolStep::Idle
            }
            None => ToolStep::Idle,
        }
    }

    /// 🧊️ Host cancel of every open scrub (a time-travel freeze): zero trace. Answers one `Aborted` step per scrub.
    pub fn abort_all(&mut self, reason: ToolAbortReason) -> Vec<ToolStep<M>> {
        let windows: Vec<String> = self.windows.keys().cloned().collect();
        windows.iter().map(|window| self.abort(window, None, reason)).collect()
    }

    /// 🪦️ Host cancel (`retired`) of the open scrub of every window `keep` refuses; their closed presses are forgotten.
    pub fn retain_windows(&mut self, keep: impl Fn(&str) -> bool) -> Vec<ToolStep<M>> {
        let retired: Vec<String> = self.windows.keys().filter(|window| !keep(window)).cloned().collect();
        let dropped = retired.iter().map(|window| self.abort(window, None, ToolAbortReason::Retired)).collect();
        self.closed.retain(|window, _| keep(window));
        dropped
    }
}
//#endregion 🔖️Scrub

//#region 🔖️NodeDrag
/// ✋️ The `nodeGraphEdit` row operation of a released node drag (design §13.3).
pub const NODE_DRAG_OPERATION: &str = "move";
/// 🧾️ The closed field set of a [`NODE_DRAG_OPERATION`] row.
pub const NODE_DRAG_ROW_FIELDS: [&str; 5] = ["operation", "gestureId", "nodeIds", "dx", "dy"];

/// ✋️ The node-graph gesture record every node-graph host dispatches for a released node drag (design §13.3): the press
/// it closes, the nodes it moved, and the ONE offset every one of them moved by, relative to where it started. A guest
/// turns it into its own relative leaf (`drag-nodes`, `move-nodes`, …) and commits it through [`node_drag_commit`], so
/// editing the drag in history replays the offset on whatever base it lands on.
#[derive(Clone, Debug, PartialEq)]
pub struct NodeDragRecord {
    pub gesture_id: String,
    pub node_ids: Vec<String>,
    pub dx: f64,
    pub dy: f64,
}

impl NodeDragRecord {
    /// 🧾️ Decodes one `{operation:"move", gestureId, nodeIds, dx, dy}` row; any other field, a missing one, an empty
    /// gesture or node id, no node or one node twice, or a non-finite offset is refused by name.
    pub fn from_row(row: &protocol::DslValue) -> Result<Self, String> {
        let fields = row.as_object().ok_or("a node drag row must be an object")?;
        if fields.len() != NODE_DRAG_ROW_FIELDS.len() || NODE_DRAG_ROW_FIELDS.iter().any(|field| fields.iter().filter(|(name, _)| name == field).count() != 1) {
            return Err(format!("a node drag row has exactly the fields {NODE_DRAG_ROW_FIELDS:?}"));
        }
        if row.get("operation").and_then(protocol::DslValue::as_str) != Some(NODE_DRAG_OPERATION) {
            return Err(format!("a node drag row is the `{NODE_DRAG_OPERATION}` operation"));
        }
        let gesture_id = row.get("gestureId").and_then(protocol::DslValue::as_str).filter(|id| !id.is_empty()).ok_or("a node drag row names its gestureId")?.to_string();
        let node_ids = row
            .get("nodeIds")
            .and_then(protocol::DslValue::as_array)
            .ok_or("a node drag row lists its nodeIds")?
            .iter()
            .map(|id| id.as_str().filter(|id| !id.is_empty()).map(str::to_string).ok_or("every nodeIds entry is a non-empty node id"))
            .collect::<Result<Vec<_>, _>>()?;
        if node_ids.is_empty() || node_ids.iter().enumerate().any(|(at, id)| node_ids[..at].contains(id)) {
            return Err("a node drag row names at least one node and each node once".into());
        }
        let offset = |field: &str| row.get(field).and_then(protocol::DslValue::as_f64).filter(|value| value.is_finite()).ok_or(format!("a node drag row's {field} is a finite number"));
        Ok(Self { gesture_id, node_ids, dx: offset("dx")?, dy: offset("dy")? })
    }

    /// 📤️ The row a host writes for this record.
    pub fn to_row(&self) -> protocol::DslValue {
        protocol::DslValue::object([
            ("operation".to_string(), protocol::DslValue::String(NODE_DRAG_OPERATION.to_string())),
            ("gestureId".to_string(), protocol::DslValue::String(self.gesture_id.clone())),
            ("nodeIds".to_string(), protocol::DslValue::Array(self.node_ids.iter().cloned().map(protocol::DslValue::String).collect())),
            ("dx".to_string(), protocol::DslValue::float(self.dx)),
            ("dy".to_string(), protocol::DslValue::float(self.dy)),
        ])
    }

    /// 🎚️ Whether the record moves anything: at least one node and a finite offset that is not zero.
    pub fn moves(&self) -> bool {
        !self.node_ids.is_empty() && self.dx.is_finite() && self.dy.is_finite() && (self.dx, self.dy) != (0.0, 0.0)
    }
}

/// 🛠️ The ONE node-drag machine: a released drag of the press `gesture` committed as ONE tool transaction of the guest's
/// net `leaves`, through the continuous-control [`Scrub`] (a release from rest is a one-shot press; a streaming host only
/// adds `Tick`s of the same press, each carrying the cumulative offset). The ref is minted from `actor` (the admission's
/// authoring seed), `clock` and `tool` (`<appId>#<verb>`). `None` when nothing is yielded: an empty drag leaves zero trace.
pub fn node_drag_commit<M: Clone + 'static>(tool: impl Into<String>, actor: ActorId, gesture: &str, leaves: Vec<M>, clock: HybridLogicalTimestamp) -> Option<(TransactionRef, Vec<M>)> {
    if leaves.is_empty() {
        return None;
    }
    match Scrub::start(tool, actor, "").send(ScrubInput::Commit { gesture: gesture.to_string(), leaves }, clock).ok()? {
        ToolStep::Committed(transaction, mutations) => Some((transaction, mutations)),
        ToolStep::Idle | ToolStep::Open | ToolStep::Aborted(..) | ToolStep::Empty(_) => None,
    }
}
//#endregion 🔖️NodeDrag

//#region 🔖️NodeGraphEditRows
/// 📏️ The most rows one `nodeGraphEdit` dispatch carries.
pub const NODE_GRAPH_EDIT_MAX_ROWS: usize = 256;
/// 🧾️ The root fields a `nodeGraphEdit` dispatch may carry: its rows, and the press of a dragged inline slider that the
/// framework scrub machine reads (design §13.1).
pub const NODE_GRAPH_EDIT_ROOT_FIELDS: [&str; 4] = ["operations", SCRUB_GESTURE_ARG, SCRUB_COMMIT_ARG, SCRUB_ABORT_ARG];

/// 🔌️ The side of a node a variadic port is inserted on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodePortSide {
    Input,
    Output,
}

/// 🔗️ One node-graph gesture record every renderer dispatches as `nodeGraphEdit` arguments (design §13.3; schema
/// `📺️renderer/🧑‍🎨engine/🧬️schema/🔣️node-graph-edit-rows`): each row names its entities by id and carries an intent, never a
/// whole fixture. A guest maps every row to its own id-keyed leaf; adding a node stays each guest's own verb.
#[derive(Clone, Debug, PartialEq)]
pub enum NodeGraphEditRow {
    Connect { source_node_id: String, source_port_id: String, target_node_id: String, target_port_id: String },
    Disconnect { synapse_id: String },
    Move(NodeDragRecord),
    SetSlider { widget_id: String, value: f64 },
    InsertPort { node_id: String, side: NodePortSide, index: u32 },
    Delete { node_ids: Vec<String>, synapse_ids: Vec<String> },
}

impl NodeGraphEditRow {
    /// 🧾️ Decodes one closed row; an unknown operation, a field outside the row's schema, an empty id, a repeated id, a
    /// non-numeric or non-finite number, an unknown port side or a non-integer index is refused by name.
    pub fn from_row(row: &protocol::DslValue) -> Result<Self, String> {
        let operation = row.get("operation").and_then(protocol::DslValue::as_str).ok_or("a nodeGraphEdit row names its operation")?;
        let closed = |expected: &[&str]| {
            let fields = row.as_object().ok_or(format!("a nodeGraphEdit {operation} row must be an object"))?;
            match fields.len() == expected.len() && expected.iter().all(|field| fields.iter().filter(|(name, _)| name == field).count() == 1) {
                true => Ok(()),
                false => Err(format!("a nodeGraphEdit {operation} row has exactly the fields {expected:?}")),
            }
        };
        let text = |field: &str| row.get(field).and_then(protocol::DslValue::as_str).map(str::to_string).ok_or(format!("nodeGraphEdit {operation}.{field} must be a string"));
        let id = |field: &str| text(field).and_then(|value| if value.is_empty() { Err(format!("nodeGraphEdit {operation}.{field} must not be empty")) } else { Ok(value) });
        let ids = |field: &str| {
            let ids = row
                .get(field)
                .and_then(protocol::DslValue::as_array)
                .ok_or(format!("nodeGraphEdit {operation}.{field} must be an array"))?
                .iter()
                .map(|id| id.as_str().filter(|id| !id.is_empty()).map(str::to_string).ok_or(format!("nodeGraphEdit {operation}.{field} holds non-empty ids")))
                .collect::<Result<Vec<_>, _>>()?;
            match ids.iter().enumerate().any(|(at, id)| ids[..at].contains(id)) {
                true => Err(format!("nodeGraphEdit {operation}.{field} names each id once")),
                false => Ok(ids),
            }
        };
        match operation {
            "connect" => {
                closed(&["operation", "sourceNodeId", "sourcePortId", "targetNodeId", "targetPortId"])?;
                Ok(Self::Connect { source_node_id: id("sourceNodeId")?, source_port_id: text("sourcePortId")?, target_node_id: id("targetNodeId")?, target_port_id: text("targetPortId")? })
            }
            "disconnect" => {
                closed(&["operation", "synapseId"])?;
                Ok(Self::Disconnect { synapse_id: id("synapseId")? })
            }
            NODE_DRAG_OPERATION => NodeDragRecord::from_row(row).map(Self::Move),
            "setSlider" => {
                closed(&["operation", "widgetId", "value"])?;
                let value = row.get("value").and_then(protocol::DslValue::as_f64).filter(|value| value.is_finite()).ok_or("nodeGraphEdit setSlider.value must be a finite number")?;
                Ok(Self::SetSlider { widget_id: id("widgetId")?, value })
            }
            "insertPort" => {
                closed(&["operation", "nodeId", "side", "index"])?;
                let side = match text("side")?.as_str() {
                    "input" => NodePortSide::Input,
                    "output" => NodePortSide::Output,
                    other => return Err(format!("nodeGraphEdit insertPort.side is input or output, not {other:?}")),
                };
                let index = row.get("index").and_then(protocol::DslValue::as_f64).filter(|index| index.fract() == 0.0 && (0.0..=f64::from(u32::MAX)).contains(index)).ok_or("nodeGraphEdit insertPort.index must be a non-negative integer")?;
                Ok(Self::InsertPort { node_id: id("nodeId")?, side, index: index as u32 })
            }
            "delete" => {
                closed(&["operation", "nodeIds", "synapseIds"])?;
                let (node_ids, synapse_ids) = (ids("nodeIds")?, ids("synapseIds")?);
                match node_ids.is_empty() && synapse_ids.is_empty() {
                    true => Err("nodeGraphEdit delete names at least one node or synapse".into()),
                    false => Ok(Self::Delete { node_ids, synapse_ids }),
                }
            }
            other => Err(format!("nodeGraphEdit has no operation {other:?}")),
        }
    }
}

/// 🧾️ Decodes the arguments of one `nodeGraphEdit` dispatch: an object holding exactly its `operations` (at most
/// [`NODE_GRAPH_EDIT_MAX_ROWS`] closed rows) and optionally the scrub press fields. Any malformed row refuses the whole batch
/// before a guest maps anything; an empty batch (a host abort of a slider press) decodes to no rows.
pub fn node_graph_edit_rows(args: &protocol::DslValue) -> Result<Vec<NodeGraphEditRow>, String> {
    let root = args.as_object().ok_or("nodeGraphEdit arguments must be an object")?;
    if root.iter().any(|(field, _)| !NODE_GRAPH_EDIT_ROOT_FIELDS.contains(&field.as_str())) || root.iter().filter(|(field, _)| field == "operations").count() != 1 {
        return Err(format!("nodeGraphEdit arguments hold exactly an operations array and optionally {:?}", &NODE_GRAPH_EDIT_ROOT_FIELDS[1..]));
    }
    let rows = args.get("operations").and_then(protocol::DslValue::as_array).ok_or("nodeGraphEdit operations must be an array")?;
    if rows.len() > NODE_GRAPH_EDIT_MAX_ROWS {
        return Err(format!("nodeGraphEdit carries at most {NODE_GRAPH_EDIT_MAX_ROWS} rows"));
    }
    rows.iter().map(NodeGraphEditRow::from_row).collect()
}
//#endregion 🔖️NodeGraphEditRows

//#region 🔖️Typing
/// ⌨️ The argument naming the text buffer a live typing delivery types into (its editor surface id); a dispatch without it is a
/// plain one-shot edit (an agent's or a palette's).
pub const TYPING_BUFFER_ARG: &str = "typing";
/// 🏁️ The argument of a host's run commit signal ([`TypingCommit`]); such a dispatch carries no edit.
pub const TYPING_COMMIT_ARG: &str = "typingCommit";
/// ⏱️ How long a typing run stays open after its last edit before it commits on its own.
pub const TYPING_IDLE_MS: u64 = 750;

/// 🏁️ Why a typing run ended as ONE edit: the author paused, the caret jumped away, the editor lost focus, Enter in a
/// single-line field, the page was hidden or left, an explicit apply, or another verb needs the typed text first.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TypingCommit {
    Idle,
    SelectionJump,
    Blur,
    Enter,
    Hidden,
    Apply,
    OtherVerb,
}

impl TypingCommit {
    pub const ALL: [Self; 7] = [Self::Idle, Self::SelectionJump, Self::Blur, Self::Enter, Self::Hidden, Self::Apply, Self::OtherVerb];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::SelectionJump => "selectionJump",
            Self::Blur => "blur",
            Self::Enter => "enter",
            Self::Hidden => "hidden",
            Self::Apply => "apply",
            Self::OtherVerb => "otherVerb",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|reason| reason.as_str() == text)
    }
}

/// ⌨️ Where one dispatch sits in its buffer's typing run: a typed edit, or the host's commit signal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypingPhase {
    Edit { buffer: String },
    Commit { buffer: String, reason: TypingCommit },
}

impl TypingPhase {
    /// 🧩️ Reads the typing arguments: `None` without a non-empty `typing` buffer (a one-shot dispatch) or with an unknown commit
    /// reason.
    pub fn parse(buffer: Option<&str>, commit: Option<&str>) -> Option<Self> {
        let buffer = buffer.filter(|buffer| !buffer.is_empty())?.to_string();
        match commit {
            Some(reason) => TypingCommit::parse(reason).map(|reason| Self::Commit { buffer, reason }),
            None => Some(Self::Edit { buffer }),
        }
    }

    /// 🆔️ The buffer the dispatch types into.
    pub fn buffer(&self) -> &str {
        match self {
            Self::Edit { buffer } | Self::Commit { buffer, .. } => buffer,
        }
    }
}

/// 🔗️ How the leaves of one typed edit join the open run (an app's typing algebra): the run's new net leaves, or a split — the
/// open run commits as it is and the edit opens the next run.
#[derive(Clone, Debug, PartialEq)]
pub enum TypingFold<M> {
    Net(Vec<M>),
    Split,
}

/// 📨️ What reaches a typing run: one typed edit's leaves, a commit, or a host abort (`baseMoved` conflict, `frozen`).
#[derive(Clone, Debug, PartialEq)]
pub enum TypingInput<M> {
    Edit { buffer: String, leaves: Vec<M> },
    Commit { reason: TypingCommit },
    Abort { reason: ToolAbortReason },
}

/// 🧰️ A typing run's tool state: the buffer it types into and how many keyed net leaves (`"0"`, `"1"`, …) its transaction holds.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TypingContext {
    pub buffer: Option<String>,
    pub keys: usize,
}

/// 📨️ The typing statechart's events (the idle lapse is its `after` timer, a host abort the runner's
/// [`ToolMachineRunner::abort`]).
#[derive(Clone, Debug, PartialEq)]
pub enum TypingEvent<M> {
    Edit { buffer: String, leaves: Vec<M> },
    Commit { reason: TypingCommit },
}

impl<M: Clone> machine::StatechartEvent for TypingEvent<M> {
    const EVENT_COUNT: u16 = 2;

    fn event_id(&self) -> machine::EventId {
        match self {
            Self::Edit { .. } => machine::EventId(0),
            Self::Commit { .. } => machine::EventId(1),
        }
    }

    fn event_name(id: machine::EventId) -> &'static str {
        match id.0 {
            0 => "Edit",
            1 => "Commit",
            _ => "?",
        }
    }
}

/// ⌨️ The ONE typing tool: `idle → typing` on an edit, every edit of the same buffer replaces the run's net leaves (upsert by
/// position, retract the rest) and re-arms the idle timer, which commits the run [`TYPING_IDLE_MS`] after its last edit; a
/// commit signal ends the run as ONE edit. Typing never aborts on its own: only a host abort (a conflicting base, a frozen
/// document) drops a run with zero trace. Tables are M-independent and pinned against the `statechart!` compilation of the
/// same chart by the unit laws.
pub struct TypingMachine<M>(std::marker::PhantomData<fn() -> M>);

const TYPING_NODES: [machine::NodeDef; 3] = [
    machine::NodeDef { stable_id: "root", kind: machine::NodeKind::Compound, parent: None, initial: Some(machine::NodeId(1)), children: &[machine::NodeId(1), machine::NodeId(2)], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 0 },
    machine::NodeDef { stable_id: "idle", kind: machine::NodeKind::Atomic, parent: Some(machine::NodeId(0)), initial: None, children: &[], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[], doc_index: 1 },
    machine::NodeDef { stable_id: "typing", kind: machine::NodeKind::Atomic, parent: Some(machine::NodeId(0)), initial: None, children: &[], entry_actions: &[], exit_actions: &[], invokes: &[], timers: &[(TYPING_IDLE_TIMER, TYPING_IDLE_MS)], doc_index: 2 },
];

const TYPING_TRANSITIONS: [machine::TransitionDef; 4] = [
    machine::TransitionDef { source: machine::NodeId(1), trigger: machine::Trigger::Event(machine::EventId(0)), guard: None, targets: &[machine::NodeId(2)], kind: machine::TransitionKind::External, actions: &[machine::ActionId(0)], doc_index: 0 },
    machine::TransitionDef { source: machine::NodeId(2), trigger: machine::Trigger::Timer(TYPING_IDLE_TIMER), guard: None, targets: &[machine::NodeId(1)], kind: machine::TransitionKind::External, actions: &[machine::ActionId(1)], doc_index: 1 },
    machine::TransitionDef { source: machine::NodeId(2), trigger: machine::Trigger::Event(machine::EventId(0)), guard: Some(machine::GuardId(0)), targets: &[machine::NodeId(2)], kind: machine::TransitionKind::External, actions: &[machine::ActionId(0)], doc_index: 2 },
    machine::TransitionDef { source: machine::NodeId(2), trigger: machine::Trigger::Event(machine::EventId(1)), guard: None, targets: &[machine::NodeId(1)], kind: machine::TransitionKind::External, actions: &[machine::ActionId(1)], doc_index: 3 },
];

/// ⏱️ The typing chart's one `after` timer: the idle lapse of the `typing` state.
pub const TYPING_IDLE_TIMER: TimerId = TimerId(0);
/// 🔏️ The `statechart!` fingerprint of the typing chart (restore gate of a persisted run).
pub const TYPING_FINGERPRINT: u64 = 18324688487390181293;
/// 🗺️ The `statechart!` manifest of the typing chart.
pub const TYPING_MANIFEST_JSON: &str = r#"{"id":"typing","states":[{"id":"root","parent":null},{"id":"idle","parent":0},{"id":"typing","parent":0}],"events":["Edit","Commit"],"transitionCount":4}"#;

impl<M: Clone + 'static> TypingMachine<M> {
    const DEFINITION: MachineDefinition<Self> = MachineDefinition {
        id: "typing",
        nodes: &TYPING_NODES,
        transitions: &TYPING_TRANSITIONS,
        context_from_input: typing_context,
        make_output: None,
        guards: &[typing_same_buffer::<M>],
        actions: &[typing_follow::<M>, typing_settle::<M>],
        fingerprint: TYPING_FINGERPRINT,
        manifest_json: TYPING_MANIFEST_JSON,
    };
}

impl<M: Clone + 'static> Machine for TypingMachine<M> {
    type Context = TypingContext;
    type Event = TypingEvent<M>;
    type Input = TypingContext;
    type Output = ();
    type Effect = ToolYield<M>;
    type Config = machine::BitSet<1>;

    fn definition() -> &'static MachineDefinition<Self> {
        &Self::DEFINITION
    }
}

fn typing_context(input: TypingContext) -> TypingContext {
    input
}

fn typing_same_buffer<M>(context: &TypingContext, event: Option<&TypingEvent<M>>) -> bool {
    matches!(event, Some(TypingEvent::Edit { buffer, .. }) if context.buffer.as_deref() == Some(buffer.as_str()))
}

fn typing_follow<M: Clone + 'static>(context: &mut TypingContext, event: Option<&TypingEvent<M>>, sink: &mut Vec<Command<TypingMachine<M>>>) {
    let Some(TypingEvent::Edit { buffer, leaves }) = event else { return };
    context.buffer = Some(buffer.clone());
    sink.extend(leaves.iter().enumerate().map(|(index, leaf)| Command::Effect(ToolYield::upsert(index.to_string(), leaf.clone()))));
    sink.extend((leaves.len()..context.keys).map(|index| Command::Effect(ToolYield::retract(index.to_string()))));
    context.keys = leaves.len();
}

fn typing_settle<M: Clone + 'static>(context: &mut TypingContext, _event: Option<&TypingEvent<M>>, sink: &mut Vec<Command<TypingMachine<M>>>) {
    sink.push(Command::Effect(ToolYield::Commit));
    *context = TypingContext::default();
}

/// ⏰️ The typing run's host: its clock is the clock of the input being run, and the one `after` timer is a deadline the owner
/// checks ([`Typing::lapse`]) — a pure host, no wall clock, no callback.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TypingHost {
    pub now_ms: u64,
    pub deadline_ms: Option<u64>,
}

impl<M: Clone + 'static> Host<TypingMachine<M>> for TypingHost {
    fn execute_effect(&mut self, _actor: machine::ActorId, _effect: ToolYield<M>) {}
    fn schedule(&mut self, _actor: machine::ActorId, _timer: TimerId, delay_ms: u64) {
        self.deadline_ms = Some(self.now_ms.saturating_add(delay_ms));
    }
    fn cancel_timer(&mut self, _actor: machine::ActorId, _timer: TimerId) {
        self.deadline_ms = None;
    }
    fn start_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn cancel_task(&mut self, _actor: machine::ActorId, _invoke: machine::InvokeId) {}
    fn now_ms(&self) -> u64 {
        self.now_ms
    }
}

/// 💾️ One window's open typing run between dispatches (window transient, ephemeral local-only, never history): the
/// configuration by stable ids, the authoring tool `<appId>#<verb>` and actor, the buffer, the idle deadline, and the open
/// transaction with its net leaves.
#[derive(Clone, Debug, PartialEq)]
pub struct TypingState<M> {
    pub states: Vec<String>,
    pub tool: String,
    pub actor: String,
    pub buffer: String,
    pub deadline_ms: u64,
    pub transaction: TransactionRef,
    pub entries: Vec<(String, M)>,
}

/// ⌨️ One typing run: [`TypingMachine`] under a [`ToolMachineRunner`]. Every input runs on its own clock, which must be unique
/// per author and tool (the run's id is minted from it); one input mints at most one transaction.
pub struct Typing<M: Clone + 'static> {
    runner: ToolMachineRunner<TypingMachine<M>, TypingHost>,
}

impl<M: Clone + 'static> Typing<M> {
    /// 🚀️ A run at rest for `tool` (`<appId>#<verb>`) by `actor`.
    pub fn start(tool: impl Into<String>, actor: ActorId) -> Self {
        let runner = ToolMachineRunner::start(tool, actor, TypingContext::default(), TypingHost::default()).expect("the typing chart yields nothing while entering");
        Self { runner }
    }

    /// ⏯️ The run a window persisted, restored by stable ids with its open transaction and idle deadline; a state the chart
    /// cannot restore is refused (`Closed`).
    pub fn resume(state: TypingState<M>) -> Result<Self, ToolRefusal> {
        let persisted = machine::PersistedSnapshot { version: 1, fingerprint: TYPING_FINGERPRINT, states: state.states, history: Vec::new(), done: false };
        let context = TypingContext { buffer: Some(state.buffer), keys: state.entries.len() };
        let snapshot = machine::restore::<TypingMachine<M>, machine::NoMigrations>(&persisted, context, &[]).map_err(|_| ToolRefusal::Closed)?;
        let transaction = ToolTransaction::resume(state.transaction, state.entries);
        let host = TypingHost { now_ms: 0, deadline_ms: Some(state.deadline_ms) };
        let runner = ToolMachineRunner::resume(state.tool, ActorId(state.actor), TypingContext::default(), snapshot, Some(transaction), host)?;
        Ok(Self { runner })
    }

    /// 🆔️ The buffer the open run types into.
    pub fn buffer(&self) -> Option<&str> {
        self.runner.snapshot().context.buffer.as_deref()
    }

    /// ⏱️ When the open run commits on its own.
    pub fn deadline_ms(&self) -> Option<u64> {
        self.runner.host.deadline_ms
    }

    /// 🔧️ The authoring tool id.
    pub fn tool(&self) -> &str {
        self.runner.tool()
    }

    /// 📝️ The open transaction: the run's net leaves the preview overlays.
    pub fn transaction(&self) -> Option<&ToolTransaction<M>> {
        self.runner.transaction()
    }

    /// ⏱️ Fires the idle lapse when `clock` reached the deadline: the run commits as ONE edit. `None` when nothing lapsed.
    pub fn lapse(&mut self, clock: HybridLogicalTimestamp) -> Result<Option<ToolStep<M>>, ToolRefusal> {
        self.runner.host.now_ms = clock.physical_ms;
        match self.runner.host.deadline_ms {
            Some(deadline) if deadline <= clock.physical_ms => self.runner.timer_elapsed(TYPING_IDLE_TIMER, clock).map(Some),
            _ => Ok(None),
        }
    }

    /// 📨️ Runs one input on `clock`. An edit first lets a lapsed run commit, then folds into the open run of its buffer through
    /// `fold`; an edit of another buffer, or one `fold` splits off, commits the open run first (`selectionJump`) and opens the
    /// next. Answers every step in order: an edit yields at most one commit before its own step.
    pub fn send(&mut self, input: TypingInput<M>, fold: impl Fn(&[M], &[M]) -> TypingFold<M>, clock: HybridLogicalTimestamp) -> Result<Vec<ToolStep<M>>, ToolRefusal> {
        self.runner.host.now_ms = clock.physical_ms;
        let (buffer, leaves) = match input {
            TypingInput::Abort { reason } => return Ok(vec![self.runner.abort(reason)]),
            TypingInput::Commit { reason } => return Ok(vec![self.runner.send(TypingEvent::Commit { reason }, clock)?]),
            TypingInput::Edit { buffer, leaves } => (buffer, leaves),
        };
        let mut steps: Vec<ToolStep<M>> = self.lapse(clock)?.into_iter().collect();
        let net = match (self.buffer(), self.transaction()) {
            (Some(open), _) if open != buffer => TypingFold::Split,
            (Some(_), Some(transaction)) => fold(&transaction.entries().iter().map(|(_, leaf)| leaf.clone()).collect::<Vec<_>>(), &leaves),
            _ => TypingFold::Net(leaves.clone()),
        };
        let leaves = match net {
            TypingFold::Net(net) => net,
            TypingFold::Split => {
                steps.push(self.runner.send(TypingEvent::Commit { reason: TypingCommit::SelectionJump }, clock)?);
                leaves
            }
        };
        steps.push(self.runner.send(TypingEvent::Edit { buffer, leaves }, clock)?);
        Ok(steps)
    }

    /// 💾️ The state to persist: `Some` only while a transaction is open.
    pub fn persist(self) -> Option<TypingState<M>> {
        let (tool, actor, deadline_ms) = (self.runner.tool().to_string(), self.runner.actor().0.clone(), self.runner.host.deadline_ms?);
        let (snapshot, transaction) = self.runner.into_parts();
        let transaction = transaction.filter(|transaction| transaction.state() == ToolTransactionState::Open)?;
        let buffer = snapshot.context.buffer.clone()?;
        Some(TypingState { states: machine::persist(&snapshot).states, tool, actor, buffer, deadline_ms, transaction: transaction.reference().clone(), entries: transaction.entries().to_vec() })
    }
}

/// 🗂️ Every window's open typing run (at most one per window). Pure: the runtime keeps one per app instance, overlays
/// [`Self::provisional`] on the committed document for every render, publishes every committed run as ONE edit stamped with
/// its `TransactionRef`, and fires lapsed runs ([`Self::lapse`]) whenever it runs.
#[derive(Clone, Debug, PartialEq)]
pub struct TypingLedger<M> {
    windows: std::collections::BTreeMap<String, TypingState<M>>,
}

impl<M> Default for TypingLedger<M> {
    fn default() -> Self {
        Self { windows: std::collections::BTreeMap::new() }
    }
}

impl<M: Clone + 'static> TypingLedger<M> {
    /// 🛋️ Whether no window types.
    pub fn is_empty(&self) -> bool {
        self.windows.is_empty()
    }

    /// 🔎️ The open run of `window`.
    pub fn open(&self, window: &str) -> Option<&TypingState<M>> {
        self.windows.get(window)
    }

    /// 🪟️ The windows holding an open run, in window id order.
    pub fn windows(&self) -> impl Iterator<Item = &str> {
        self.windows.keys().map(String::as_str)
    }

    /// 👁️ Every open run's net leaves, window by window in window id order — the overlay a render applies on the committed
    /// document; never history.
    pub fn provisional(&self) -> impl Iterator<Item = &M> {
        self.windows.values().flat_map(|state| state.entries.iter().map(|(_, leaf)| leaf))
    }

    /// 🔜️ The earliest idle deadline of any open run.
    pub fn next_deadline_ms(&self) -> Option<u64> {
        self.windows.values().map(|state| state.deadline_ms).min()
    }

    fn run(&mut self, window: &str, tool: &str, actor: &ActorId) -> (Typing<M>, Option<ToolStep<M>>) {
        match self.windows.remove(window).map(Typing::resume) {
            Some(Ok(typing)) if typing.tool() == tool => (typing, None),
            Some(Ok(mut other)) => {
                let step = other.runner.send(TypingEvent::Commit { reason: TypingCommit::SelectionJump }, HybridLogicalTimestamp { actor: 0, physical_ms: other.runner.host.now_ms, logical: 0 }).ok();
                (Typing::start(tool, actor.clone()), step)
            }
            _ => (Typing::start(tool, actor.clone()), None),
        }
    }

    fn keep(&mut self, window: &str, typing: Typing<M>) {
        if let Some(state) = typing.persist() {
            self.windows.insert(window.to_string(), state);
        }
    }

    /// 📨️ Runs one input of `window`'s run on `clock`: an open run of another tool in the window commits first (`selectionJump`).
    /// Answers every step in order.
    pub fn send(&mut self, window: &str, tool: &str, actor: &ActorId, input: TypingInput<M>, fold: impl Fn(&[M], &[M]) -> TypingFold<M>, clock: HybridLogicalTimestamp) -> Result<Vec<ToolStep<M>>, ToolRefusal> {
        let (mut typing, other) = self.run(window, tool, actor);
        let steps = typing.send(input, fold, clock);
        self.keep(window, typing);
        steps.map(|steps| other.into_iter().chain(steps).collect())
    }

    /// 🏁️ Commits `window`'s open run as ONE edit (`reason`); `Idle` when the window does not type.
    pub fn commit(&mut self, window: &str, reason: TypingCommit, clock: HybridLogicalTimestamp) -> Result<ToolStep<M>, ToolRefusal> {
        let Some(state) = self.windows.remove(window) else { return Ok(ToolStep::Idle) };
        let mut typing = Typing::resume(state)?;
        typing.runner.host.now_ms = clock.physical_ms;
        let step = typing.runner.send(TypingEvent::Commit { reason }, clock);
        self.keep(window, typing);
        step
    }

    /// 🏁️ Commits every open run (another verb needs the typed text first, the page is left): one step per window.
    pub fn commit_all(&mut self, reason: TypingCommit, clock: HybridLogicalTimestamp) -> Vec<(String, Result<ToolStep<M>, ToolRefusal>)> {
        let windows: Vec<String> = self.windows.keys().cloned().collect();
        windows.into_iter().map(|window| (window.clone(), self.commit(&window, reason, clock))).collect()
    }

    /// ⏱️ Fires the idle lapse of every run whose deadline `clock` reached: each commits as ONE edit, one step per lapsed window.
    pub fn lapse(&mut self, clock: HybridLogicalTimestamp) -> Vec<(String, Result<ToolStep<M>, ToolRefusal>)> {
        let lapsed: Vec<String> = self.windows.iter().filter(|(_, state)| state.deadline_ms <= clock.physical_ms).map(|(window, _)| window.clone()).collect();
        lapsed
            .into_iter()
            .filter_map(|window| {
                let mut typing = match Typing::resume(self.windows.remove(&window)?) {
                    Ok(typing) => typing,
                    Err(refusal) => return Some((window, Err(refusal))),
                };
                let step = typing.lapse(clock).map(|step| step.unwrap_or(ToolStep::Idle));
                self.keep(&window, typing);
                Some((window, step))
            })
            .collect()
    }

    /// 🧯️ Host abort of `window`'s open run (a conflicting base): zero trace. `Aborted(ref, reason)`, or `Idle`.
    pub fn abort(&mut self, window: &str, reason: ToolAbortReason) -> ToolStep<M> {
        self.windows.remove(window).map_or(ToolStep::Idle, |state| ToolStep::Aborted(state.transaction, reason))
    }

    /// 🧊️ Host abort of every open run (a frozen document): zero trace, one step per window.
    pub fn abort_all(&mut self, reason: ToolAbortReason) -> Vec<(String, ToolStep<M>)> {
        let windows: Vec<String> = self.windows.keys().cloned().collect();
        windows.into_iter().map(|window| (window.clone(), self.abort(&window, reason))).collect()
    }

    /// 🪦️ A window that left the roster ends its run like a blur: the typed text commits as ONE edit.
    pub fn retain_windows(&mut self, keep: impl Fn(&str) -> bool, clock: HybridLogicalTimestamp) -> Vec<(String, Result<ToolStep<M>, ToolRefusal>)> {
        let retired: Vec<String> = self.windows.keys().filter(|window| !keep(window)).cloned().collect();
        retired.into_iter().map(|window| (window.clone(), self.commit(&window, TypingCommit::Blur, clock))).collect()
    }
}
//#endregion 🔖️Typing

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️node-graph-edit-rows/🦀️.rs"]
mod node_graph_edit_rows_tests;
//#endregion 🧪️Tests

```

### 🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust/📜️script.ts

SHA-256 da85e8bcdc17c21bf119a176370aab5f6c9c62e625ce3442e14ab81e88a7cf20; 2209 bytes.

```typescript
#!/usr/bin/env bun
import { runOwnedCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { buildBudgetMs } from "../../../🏃️process/⏱️budget/🟦️.ts";
import { resolve } from "node:path";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { runBudgetedTestCommand } from "../../../🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import { testLevelBudgetMs, resolveTestLevel } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🛠️ `@semio-tech/framework-tool-machine-rs` router: `bun ./📜️script.ts <test|check>` — TS conformance (ajv/xstate/fast-check) plus Rust fixture tests, and native plus `wasm32-wasip2` type checks. */
import { join } from "node:path";

import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

const PACKAGE = "semio-framework-tool-machine";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runBudgetedTestCommand(process.execPath, ["test", join(this.root, "../../🧪️tests/🧪️conformance/🟦️.ts")], { cwd: this.repoRoot , budgetMs: testLevelBudgetMs()});
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: [PACKAGE], cwd: this.root, extraArgs: rest }, readCargoTestPolicyV1(process.env));
  }
}

class CheckScript extends BundleScript {
  async run(): Promise<void> {
    await runOwnedCommand("cargo", ["check", "--manifest-path",resolve(this.root,"Cargo.toml"), "-p", PACKAGE], this.repoRoot, "tool:owner", buildBudgetMs(), {env: process.env});
    await runOwnedCommand("cargo", ["check", "--manifest-path",resolve(this.root,"Cargo.toml"), "-p", PACKAGE, "--target", "wasm32-wasip2"], this.repoRoot, "tool:owner", buildBudgetMs(), {env: process.env});
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("check", CheckScript);

await runScriptMain(router, { defaultCommand: "test" });

```

### 🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust/📋️project.json

SHA-256 0f92d03d0f6ec418320dbe2acead6278e8a5249da3e870ffdd2135bcf25d4553; 1591 bytes.

```json
{
  "name": "@semio-tech/framework-tool-machine-rs",
  "$schema": "../../../../../node_modules/nx/schemas/project-schema.json",
  "sourceRoot": "🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust",
  "namedInputs": {
    "default": ["{workspaceRoot}/🧰️framework/🔨️modules/🛠️tool-machine/**/*", "{projectRoot}/**/*"]
  },
  "targets": {
    "test": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test"
      }
    },
    "test-quick": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test quick"
      }
    },
    "test-long": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test long"
      }
    },
    "test-exhaustive": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test exhaustive"
      }
    },
    "check": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts check"
      }
    }
  }
}

```

### 🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust/package.json

SHA-256 1e89ae28d142111debecf7df6439441d813298f0e7c63ca01ae209f876c36dd5; 342 bytes.

```json
{
  "name": "@semio-tech/framework-tool-machine-rs",
  "version": "0.1.0",
  "private": true,
  "type": "module",
  "scripts": {
    "test": "bun nx run @semio-tech/framework-tool-machine-rs:test",
    "check": "bun nx run @semio-tech/framework-tool-machine-rs:check"
  },
  "bundleKind": "library",
  "nx": {
    "includedScripts": []
  }
}

```

## Exact Documentation Corrections

```json
[
  {
    "path": "🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs",
    "beforeSha256": "2ace8b611d2efaec0ec77fa0eb37ddb2b9515df171b5d29136ae7bb1e0e69700",
    "afterSha256": "66a57e41824d23611a53d0ed0b8e2554f8738e94dc99f5d06a597b3b2beaf8f4",
    "documentationOnly": [
      [
        "/// 🧾️ The renderer's committed node-graph row vocabulary (schema `📺️renderer/🧑‍🎨engine/🧬️schema/🔣️node-graph-edit-rows`).",
        "/// 🧾️ The framework's committed node-graph row vocabulary (schema `🧰️framework/🔨️modules/🛠️tool-machine/🧬️schema/🔣️node-graph-edit-rows`)."
      ]
    ]
  },
  {
    "path": "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️node-graph/🧪️tests/🔬️unit/🦀️.rs",
    "beforeSha256": "3cebd3c7eadb50a4c9cbf529fb31ef42b9cbd6665f3fb1e8383376a0547ea755",
    "afterSha256": "91ab658f8ce7192a3e0f336aa6963d9eeb05f5e732a33831a00b4e79ab5f8d58",
    "documentationOnly": [
      [
        "/// 🧾️ The renderer's committed node-graph row vocabulary (schema `📺️renderer/🧑‍🎨engine/🧬️schema/🔣️node-graph-edit-rows`).",
        "/// 🧾️ The framework's committed node-graph row vocabulary (schema `🧰️framework/🔨️modules/🛠️tool-machine/🧬️schema/🔣️node-graph-edit-rows`)."
      ]
    ]
  },
  {
    "path": "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
    "beforeSha256": "fa9094d0ee413f386d96b346282a80347475a37d5146738641e0408a4413713d",
    "afterSha256": "2ce7b9860afa9a63b0a1c1ca86065075337c3facb318da5e5a2dbdc8abd43a7e",
    "documentationOnly": [
      [
        "/// 🧾️ The renderer's committed node-graph row vocabulary (schema `📺️renderer/🧑‍🎨engine/🧬️schema/🔣️node-graph-edit-rows`).",
        "/// 🧾️ The framework's committed node-graph row vocabulary (schema `🧰️framework/🔨️modules/🛠️tool-machine/🧬️schema/🔣️node-graph-edit-rows`)."
      ]
    ]
  },
  {
    "path": "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
    "beforeSha256": "b53bb7c9811bfd123e03dc5c4a1a9a260c405146eb227d07ae8d9fd3a35e91c4",
    "afterSha256": "6853ba1b33f59dd21cb6c5d08bd93b754280e6debb1a7d7c3a77b4b6bfd54dae",
    "documentationOnly": [
      [
        "/// `📺️renderer/🧑‍🎨engine/🧬️schema/🔣️node-graph-edit-rows`) in row order, each row to its intent leaf: `move` → the drag",
        "/// `🧰️framework/🔨️modules/🛠️tool-machine/🧬️schema/🔣️node-graph-edit-rows`) in row order, each row to its intent leaf: `move` → the drag"
      ]
    ]
  },
  {
    "path": "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
    "beforeSha256": "4a213d23e992a3ee8efd6c84dd61b08ad15bdb690f0d522a226fc0b106bae7ab",
    "afterSha256": "b693ed79a4903bc1933d668d93bcdc0bd286b4700db9056c999a59a4c0b0ad2c",
    "documentationOnly": [
      [
        "/// 🔗️ The shared node-graph record contract (`📺️renderer/🧑‍🎨engine/🧫️fixtures/🧫️node-graph-edit-rows`), as JSON.",
        "/// 🔗️ The shared node-graph record contract (`🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows`), as JSON."
      ]
    ]
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
    "beforeSha256": "03427f98643973f76bd0e7da4e7ad0a0fd06ff91226e0e3d84b9e4ffcadf7f4a",
    "afterSha256": "dcf0d500c36971fad7f6ef297928edda23ca662117492b971d654837e71426cc",
    "documentationOnly": [
      [
        "    /// it carries those ids as the `delete {nodeIds, synapseIds}` row (`📺️renderer/🧑‍🎨engine/🧬️schema/🔣️node-graph-edit-rows`).",
        "    /// it carries those ids as the `delete {nodeIds, synapseIds}` row (`🧰️framework/🔨️modules/🛠️tool-machine/🧬️schema/🔣️node-graph-edit-rows`)."
      ]
    ]
  },
  {
    "path": "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
    "beforeSha256": "a9a1cf51ff4bc3099140e0e46f044003139703cf7e68fcf0bebdd1bce9aac417",
    "afterSha256": "e8fb7bed84187a27c7441e9cc4da30e9796bd1797c89440f54645552c53ef57e",
    "documentationOnly": [
      [
        "/// `📺️renderer/🧑‍🎨engine/🧬️schema/🔣️node-graph-edit-rows`) — ONE `Emit`, so a drag is ONE undoable edit rather than one",
        "/// `🧰️framework/🔨️modules/🛠️tool-machine/🧬️schema/🔣️node-graph-edit-rows`) — ONE `Emit`, so a drag is ONE undoable edit rather than one"
      ]
    ]
  },
  {
    "path": "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
    "beforeSha256": "07c006b04ef4e93cb913ccf261587b1b68ed7273cb3a17165d71c51aae56cc6c",
    "afterSha256": "aaa87b5cfb0a5afcd00adac783934fbca66666bf3e7bdbdccd3aa66fa9997516",
    "documentationOnly": [
      [
        "/// 🔗️ Against the shared node-graph record contract (`📺️renderer/🧑‍🎨engine/🧫️fixtures/🧫️node-graph-edit-rows`): every",
        "/// 🔗️ Against the shared node-graph record contract (`🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows`): every"
      ]
    ]
  },
  {
    "path": "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs",
    "beforeSha256": "c58d8808c379ce96880935b0318dc06e525ef0fadf5066bf70e893390b6f9787",
    "afterSha256": "ed2309785c7b12f530b0721b0444b1e1221d1e20fd7bbb1605df9ae9d062ae0f",
    "documentationOnly": [
      [
        "/// 🧾️ The renderer's committed node-graph row vocabulary (schema `📺️renderer/🧑‍🎨engine/🧬️schema/🔣️node-graph-edit-rows`).",
        "/// 🧾️ The framework's committed node-graph row vocabulary (schema `🧰️framework/🔨️modules/🛠️tool-machine/🧬️schema/🔣️node-graph-edit-rows`)."
      ]
    ]
  }
]
```

## Actual Retained Input Reconstruction

All three current fixture objects match every original case value exactly. The neutral schema retains every original generic definition; the specific flow schema retains every AddWidget property and constraint. No original case or assertion was removed by the extraction. Each current Rust input is compared below to its full captured original after only explicitly declared import/documentation transforms; any other concurrent delta is reported, not attributed or reset.

```json
[
  {
    "path": "🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs",
    "sha256": "66a57e41824d23611a53d0ed0b8e2554f8738e94dc99f5d06a597b3b2beaf8f4",
    "bytes": 5380,
    "entireOriginalPreservedExceptDeclaredRouteAndDocumentation": true,
    "unattributedConcurrentDelta": false
  },
  {
    "path": "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs",
    "sha256": "ed2309785c7b12f530b0721b0444b1e1221d1e20fd7bbb1605df9ae9d062ae0f",
    "bytes": 19429,
    "entireOriginalPreservedExceptDeclaredRouteAndDocumentation": true,
    "unattributedConcurrentDelta": false
  },
  {
    "path": "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs",
    "sha256": "2e9f2908fd9a863c0ef7838f976c5821bea5855ed3ccf093b317c4e57c74a434",
    "bytes": 44716,
    "entireOriginalPreservedExceptDeclaredRouteAndDocumentation": true,
    "unattributedConcurrentDelta": false
  },
  {
    "path": "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️node-graph/🧪️tests/🔬️unit/🦀️.rs",
    "sha256": "91ab658f8ce7192a3e0f336aa6963d9eeb05f5e732a33831a00b4e79ab5f8d58",
    "bytes": 8784,
    "entireOriginalPreservedExceptDeclaredRouteAndDocumentation": true,
    "unattributedConcurrentDelta": false
  },
  {
    "path": "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
    "sha256": "301c389a1a887e528393a29df82b99d04f86552f16d20fcd1fd69f899ca5e5aa",
    "bytes": 46759,
    "entireOriginalPreservedExceptDeclaredRouteAndDocumentation": true,
    "unattributedConcurrentDelta": false
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🧪️node-graph-edit-rows/🦀️.rs",
    "sha256": "30cb9df079aa10bd93cde91e5acafdad2ba31f83d2d9bbf8a5fe85b18e3e537b",
    "bytes": 6191,
    "entireOriginalPreservedExceptDeclaredRouteAndDocumentation": true,
    "unattributedConcurrentDelta": false
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️node-graph-delete-row/🦀️.rs",
    "sha256": "30d39401133659d5d42fd334260bdcd13f94dd17d2385586107c3ed3c409e9d5",
    "bytes": 3052,
    "entireOriginalPreservedExceptDeclaredRouteAndDocumentation": true,
    "unattributedConcurrentDelta": false
  },
  {
    "path": "🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🔬️node-graph-edit-rows/🦀️.rs",
    "sha256": "3c98f3bad28fc8a67f961890fbcf7a93e10c23ed80d491e12c70f9f82e518a77",
    "bytes": 5054,
    "entireOriginalPreservedExceptDeclaredRouteAndDocumentation": true,
    "unattributedConcurrentDelta": false
  },
  {
    "path": "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
    "sha256": "2ce7b9860afa9a63b0a1c1ca86065075337c3facb318da5e5a2dbdc8abd43a7e",
    "bytes": 35873,
    "entireOriginalPreservedExceptDeclaredRouteAndDocumentation": true,
    "unattributedConcurrentDelta": false
  },
  {
    "path": "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
    "sha256": "b693ed79a4903bc1933d668d93bcdc0bd286b4700db9056c999a59a4c0b0ad2c",
    "bytes": 24859,
    "entireOriginalPreservedExceptDeclaredRouteAndDocumentation": true,
    "unattributedConcurrentDelta": false
  },
  {
    "path": "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
    "sha256": "aaa87b5cfb0a5afcd00adac783934fbca66666bf3e7bdbdccd3aa66fa9997516",
    "bytes": 23482,
    "entireOriginalPreservedExceptDeclaredRouteAndDocumentation": true,
    "unattributedConcurrentDelta": false
  },
  {
    "path": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs",
    "sha256": "650b4d33cdc3d6999a4a1c981c4fbfe8ffc8536071631eda1e6b2abd9e6a284d",
    "bytes": 13482,
    "entireOriginalPreservedExceptDeclaredRouteAndDocumentation": true,
    "unattributedConcurrentDelta": false
  },
  {
    "path": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs",
    "sha256": "98be30592dbb6ef959750f3fed95611c80409125d9f4b6ba33e31990da3a9d3d",
    "bytes": 14335,
    "entireOriginalPreservedExceptDeclaredRouteAndDocumentation": true,
    "unattributedConcurrentDelta": false
  },
  {
    "path": "🧰️framework/🔨️modules/🛠️tool-machine/🦀️.rs",
    "sha256": "dbf38b509cc2e237137f8208a3d31be6120dfd683e9e49133f3c05cbd0d26d0d",
    "bytes": 70958,
    "entireOriginalPreservedExceptDeclaredRouteAndDocumentation": true,
    "unattributedConcurrentDelta": false
  }
]
```

Registered TDD receipt: RED0pass/2fail110msBun/5.4sNx before the new fixture; GREEN2pass/0fail50assertions225msBun/4.3sNx after the extraction. Both used the original cache-bypassed normal registration; no deadlines or inference prerequisites changed. Full original native ToolMachine, actual higher consumer runtimes, and products-absent proof remain pending.

## Canonical Cache Participation

All four renderer test-level targets now explicitly include the lower ToolMachine row fixture and schema among their cache inputs; their original inputs remain. New source ownership target/router/package registration and authored launch900.05810 are present; generated launch/registry are not edited. Renderer original four-law file is under runtime verification; the current configuration does not list it in its engineTestSuites and fundamental mode selects only quickTestSuite, so the attempted selected route is not a runtime pass. Actual result will be recorded before correcting this registration.

## Original Renderer Suite Mount

The original correct long-level registered attempt is also RED: no test files found, exit1/3.4sNx/2.5scritical. The engineTestSuites roster physically omits the existing four-law row suite; fundamental-only quick selection was separately not positive evidence. Its full original config is retained below before adding exactly that existing suite to long/exhaustive participation.

### 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts

SHA-256 2f4fe328ed8a15ae44c89b1a8cdeff8504e228cc093af6a5f9c8db11d383569f; 16995 bytes.

```typescript
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { configDefaults, defineConfig } from "vitest/config";
import { repoCacheDirectory } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";

const testRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
const repoRoot = resolve(root, "../../../../../../../../../..");

const wasmEngineStub = resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts");
const testLevel = process.env.SEMIO_TEST_LEVEL ?? "fundamental";
const includeBackboneWorker = process.env.SEMIO_INCLUDE_BACKBONE_WORKER === "1";
const includeAgentBridge = process.env.SEMIO_INCLUDE_AGENT_BRIDGE === "1";
const backboneWorkerSuite = resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts");
const engineSuite = (name: string, extension = "ts") => resolve(repoRoot, `./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/${name}/🟦️.${extension}`);
// 🧱️ A suite co-located with the ELEMENT it covers, rather than under `🧑‍🎨engine/🧪️tests/`. These are not
// reachable by this package's default `include` glob either, so every one of them must be named here — a
// co-located suite that no runner includes is a gate that reads green while measuring nothing
// (`🛠️ShellHelpers/🧪️tests/🧩️component` was exactly that: the whole segmented-download drain corpus, in no
// include list at all — ticket 26/09/02 wave B38).
const elementSuite = (element: string, name: string, extension = "ts") =>
  resolve(repoRoot, `./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/${element}/🧪️tests/${name}/🟦️.${extension}`);
// 🖱️ Laws owned by the `ui` module whose only consumer is this renderer — the shell's chrome bands are
// `ui` elements, and the `ui` module declares no vitest project of its own, so a suite named nowhere else
// would never run (the same blind gate `elementSuite` exists for).
const uiSuite = (name: string, extension = "ts") => resolve(repoRoot, `./🧰️framework/🔨️modules/🖱️ui/🧪️tests/${name}/🟦️.${extension}`);
const engineTestSuites = [
  engineSuite("📄️surface-document"),
  engineSuite("🎬️initial-example", "tsx"),
  engineSuite("⚡️quick"),
  engineSuite("🎮️browser-interactive-job-port"),
  engineSuite("🏛️space-administration", "tsx"),
  engineSuite("👥️scoped-presence", "tsx"),
  engineSuite("🎚️window-measure-controls", "tsx"),
  engineSuite("🖱️world3d-interaction", "tsx"),
  engineSuite("🎥️world3d-camera-framing"),
  engineSuite("📐️projection-render-parity"),
  engineSuite("🎨️world3d-glb-outline"),
  engineSuite("🎨️world3d-glb-material"),
  engineSuite("🎨️icon-svg-lighting"),
  uiSuite("📤️prepared-readback"),
  engineSuite("🎨️world3d-scene-shading"),
  engineSuite("🚚️world3d-instance-delta"),
  engineSuite("🧊️world3d-mesh-residency"),
  engineSuite("⏱️frame-latency"),
  engineSuite("🧱️retained-frame-progress"),
  engineSuite("⏱️hop-trace"),
  engineSuite("⏱️command-stall"),
  engineSuite("🪶️surface-idle-frames", "tsx"),
  engineSuite("🪟️mounted-window-fetch"),
  engineSuite("🚪️ingress-generation-gate"),
  engineSuite("🚚️more-work-drive"),
  engineSuite("🧺️turn-patch-batch"),
  engineSuite("🎚️continuous-gesture-lane"),
  engineSuite("🎯️input-ledger"),
  engineSuite("🪟️window-host-events"),
  engineSuite("⌨️keybinding-glyphs"),
  engineSuite("⌨️text-input-oracle"),
  engineSuite("⌨️os-command-shortcuts"),
  engineSuite("🎯️world3d-pick-bounds"),
  engineSuite("📇️directory-home-bootstrap", "tsx"),
  engineSuite("📇️session-authority-notice", "tsx"),
  engineSuite("🔄️shell-utility-leaves", "tsx"),
  engineSuite("📡️actor-backbone"),
  engineSuite("📨️browser-frame-transport"),
  engineSuite("📥️wgpu-intake-budget"),
  engineSuite("📥️inbound-request"),
  engineSuite("🔬️wgpu-extension-dispatch"),
  engineSuite("🔀️surface-switch"),
  engineSuite("🧭️route-ledger"),
  engineSuite("🎭️actor-window-actions"),
  engineSuite("⌨️window-scope"),
  engineSuite("🪟️spawned-program-session", "tsx"),
  engineSuite("⌨️browser-keyboard-scope"),
  engineSuite("♿️wgpu-accessibility-interaction", "tsx"),
  engineSuite("♿️native-accessibility"),
  engineSuite("♿️retained-toggle-semantics", "tsx"),
  engineSuite("♿️editable-controls", "tsx"),
  engineSuite("📷️canvas-framing"),
  engineSuite("🆚️diff-view-produced-surface", "tsx"),
  engineSuite("🎬️playbook-scene-showcase"),
  engineSuite("🎮️wgpu-browser-input-wire"),
  engineSuite("🔬️window-host-context"),
  engineSuite("🔬️artifact-creation-ready-opening"),
  engineSuite("🔬️document-opening"),
  engineSuite("🔬️engine-contract"),
  engineSuite("🚪️opening"),
  engineSuite("🎬️activation-owner"),
  engineSuite("🎬️wasm-plugin-install"),
  engineSuite("📌️view-state-carriage"),
  engineSuite("🧩️contributions-push"),
  engineSuite("🧩️package-integration"),
  engineSuite("🧯️router-plugin-faults"),
  engineSuite("🩺️window-fault"),
  engineSuite("🫀️plugin-load-progress"),
  engineSuite("💼️job-ledger"),
  engineSuite("🪟️app-mode-layouts"),
  engineSuite("🖋️ink-canvas-editing", "tsx"),
  engineSuite("🖋️ink-canvas-domain-interaction", "tsx"),
  engineSuite("🕸️node-graph-domain-interaction", "tsx"),
  engineSuite("🖋️ink-canvas-clipboard", "tsx"),
  engineSuite("🛑️scene-pointer-cancellation"),
  engineSuite("🎬️surface-behavior"),
  engineSuite("♻️tiled-map-gesture-lifecycle"),
  engineSuite("⚙️puzzle3d-settings-document", "tsx"),
  engineSuite("🪟️maximize-resync", "tsx"),
  engineSuite("🎛️command-panel", "tsx"),
  engineSuite("🎛️retained-control-commit"),
  engineSuite("🧩️block-list-presentation", "tsx"),
  engineSuite("⏱️retained-clock", "tsx"),
  engineSuite("⏰️shell-chrome-deadline", "tsx"),
  engineSuite("⌨️caret-cadence"),
  engineSuite("⚙️settings-general-layout"),
  engineSuite("🌐️settings-locale-panel-refresh", "tsx"),
  engineSuite("🎨️settings-theme-publication", "tsx"),
  engineSuite("🎨️chrome-palette"),
  engineSuite("♻️shell-document-retirement-index"),
  engineSuite("🪗️retained-section-collapse", "tsx"),
  engineSuite("🎟️resident-refresh-budget"),
  elementSuite("🛠️ShellHelpers", "🌐️chrome-history-locale"),
  elementSuite("🛠️ShellHelpers", "🌐️instance-title", "tsx"),
  elementSuite("🛠️ShellHelpers", "🪟️empty-dock-notice", "tsx"),
  elementSuite("🖼️IconRenderHost", "🖼️frame-presentation", "tsx"),
  elementSuite("🖼️IconRenderHost", "🏷️status", "tsx"),
  elementSuite("🖼️IconRenderHost", "🚚️request", "tsx"),
  elementSuite("🖼️IconRenderHost", "📥️download"),
  elementSuite("🖼️IconRenderHost", "📤️png-export"),
  elementSuite("🖼️IconRenderHost", "📤️svg-export"),
      elementSuite("🖼️IconRenderHost", "📤️gpu-export"),
  elementSuite("🖼️IconRenderHost", "📤️export-batch"),
  elementSuite("🎬️MediaTransportHost", "♻️lifecycle", "tsx"),
  engineSuite("📤️asset-cancellation"),
  elementSuite("🖼️IconRenderHost", "⭕️svg-mask"),
  elementSuite("🛠️ShellHelpers", "🧩️component"),
  elementSuite("🛠️ShellHelpers", "🎭️browser-actor-panels"),
  elementSuite("🛠️ShellHelpers", "🧪️command-rejection"),
  elementSuite("🏛️ShellHost/📎️local-folders", "🧩️component", "tsx"),
  elementSuite("🛠️ShellHelpers", "🪟️tree-windows", "tsx"),
  elementSuite("🛠️ShellHelpers/⏯️tool-run-panel", "🧩️component", "tsx"),
  elementSuite("🛠️ShellHelpers/⏪️time-travel", "🧩️component", "tsx"),
  elementSuite("🛠️ShellHelpers", "🧪️staged-arg-controls", "tsx"),
  elementSuite("🕸️NodeGraph", "🖱️scroll-gesture"),
  elementSuite("🕸️NodeGraph", "🫱️interaction-publication"),
  elementSuite("🕸️NodeGraph", "🤏️pinch-wheel"),
  elementSuite("🧭️TiledMapHost", "🧩️component"),
  elementSuite("🧭️TiledMapHost", "🤏️pinch-gesture", "tsx"),
  elementSuite("🌐️World3dHost", "🧩️component", "tsx"),
  elementSuite("🌐️World3dHost", "🔀️projection-pane", "tsx"),
  elementSuite("🌐️World3dHost", "🤏️multi-touch", "tsx"),
  elementSuite("🌐️World3dHost/⏯️tool-run-trace", "🧩️component"),
  elementSuite("🎣️suggestion-submenu", "🧩️component"),
  elementSuite("📐️Canvas2dHost/⏯️tool-run-trace", "🧩️component"),
  elementSuite("📐️Canvas2dHost", "🧪️gumball-dispatch"),
  elementSuite("📐️Canvas2dHost", "🖱️gesture-sample-lane"),
  elementSuite("📐️Canvas2dHost", "🖊️path"),
  elementSuite("📐️Canvas2dHost", "🎨️paint"),
  elementSuite("📐️Canvas2dHost", "🧪️path-paint"),
  elementSuite("📐️Canvas2dHost", "🖱️input-contract", "tsx"),
  elementSuite("📐️Canvas2dHost", "👕️peer-presence", "tsx"),
  elementSuite("📡️EventFeedHost", "♿️accessible-entry", "tsx"),
  elementSuite("📡️EventFeedHost", "🎨️layout", "tsx"),
  elementSuite("🌳️GraphTimelineHost", "🎯️checkpoint-hit", "tsx"),
  elementSuite("🌳️GraphTimelineHost", "🎨️layout", "tsx"),
  elementSuite("📊️Table", "🔘️button-accessibility", "tsx"),
  elementSuite("🎛️UtilityTree", "🎛️picker-explicit-press", "tsx"),
  elementSuite("🖥️Board2dHost", "🧩️component"),
  elementSuite("🖥️Board2dHost", "🤏️pinch-gesture", "tsx"),
  elementSuite("🖥️Board2dHost", "👻️catalogue-drop", "tsx"),
  elementSuite("🖥️Board2dHost", "🧪️board-event-coalescing"),
  elementSuite("🖥️Board2dHost", "🧪️float32-decimal"),
  elementSuite("👕️canvas-presence", "🔬️unit"),
  elementSuite("👕️canvas-presence", "🎭️react-overlay", "tsx"),
  elementSuite("👕️canvas-presence", "✏️text-carets", "tsx"),
  elementSuite("✏️TextEditor", "🪞️echo-pack"),
  elementSuite("🖥️Board2dHost/⏯️tool-run-trace", "🧩️component"),
  elementSuite("🖌️Paint2dHost", "🔬️paint-witness"),
  elementSuite("🖌️Paint2dHost", "🧭️navigator-camera", "tsx"),
  elementSuite("🖌️Paint2dHost/✍️editing", "🖱️selection-focus", "tsx"),
  elementSuite("📃️UiDocumentStore/📥️intake", "📏️step-ceiling"),
  elementSuite("🔐️HubSignIn", "🧩️component", "tsx"),
  elementSuite("🎓️HubFirstRun", "🧩️component", "tsx"),
  elementSuite("💬️AgentChatPanel", "🧩️component", "tsx"),
  elementSuite("🤖️AgentApprovals", "🧩️component", "tsx"),
  elementSuite("🚦️AgentPresence", "🧩️component", "tsx"),
  elementSuite("🔄️ShellSync", "🧩️component", "tsx"),
  elementSuite("🧵️TaskManager", "🧩️component", "tsx"),
  elementSuite("🏘️SpaceBrowser", "🧩️component", "tsx"),
  elementSuite("🤖️AgentDelegations", "🧩️component", "tsx"),
  elementSuite("🔎️ShellSearch", "🧩️component", "tsx"),
  elementSuite("📌️ChromePanels", "🧩️component", "tsx"),
  uiSuite("🔝️navbar-centered-band"),
  uiSuite("🔤️text-advances"),
  uiSuite("📊️table-sort-header"),
] as const;
const playwrightEngineTestSuites = [engineSuite("📚️storybook-hosts-no-wasm"), engineSuite("📚️storybook-hosts-wasm")] as const;
const rootPolicySelfTestSuites = ["interactivity-live-reconcile", "interactivity-mounted-engine-surface-lifetime", "interactivity-mounted-frame-transaction"].map((id) => resolve(repoRoot, `./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️${id}/🟦️.ts`));
const quickTestSuite = resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⚡️quick/🟦️.ts");
const agentBridgeTestSuite = resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🧪️tests/🧩️component/🟦️.ts");
const longInSourceSuites = [
  resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts"),
  resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx"),
  resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx"),
  resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎚️UiPreferences/🟦️.ts"),
] as const;
const exhaustiveInSourceSuites = [
  ...longInSourceSuites,
  resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🟦️.tsx"),
] as const;

export default defineConfig({
  root: testRoot,
  cacheDir: repoCacheDirectory(repoRoot, "vite", "renderer-react"),
  resolve: {
    alias: [
      { find: "@semio-tech/ui-react/test", replacement: resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🖌️render/🟦️.ts") },
      { find: "@semio-tech/ui-react", replacement: resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx") },
      { find: "@semio-tech/assets", replacement: resolve(repoRoot, "./🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/🟦️.ts") },
      { find: "@semio-tech/ui-styling", replacement: resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript") },
      { find: "@semio-tech/framework-os", replacement: resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🟦️.ts") },
      { find: "@semio-tech/framework-surface-rs", replacement: wasmEngineStub },
      { find: "@semio-tech/framework-editor-rs", replacement: wasmEngineStub },
      { find: "@semio-tech/framework", replacement: resolve(repoRoot, "./🧰️framework/📦️packages/🟦️typescript/🟦️.ts") },
      { find: "@semio-tech/infinite-canvas-react-renderer", replacement: resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript/🟦️.tsx") },
      { find: "@semio-tech/infinite-world-r3f", replacement: resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/🟦️.tsx") },
      { find: "@semio-tech/flow-core/🌐️flow-browser.js", replacement: resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🌐️browser/🏃️runtime/🟨️.js") },
      { find: "@semio-tech/flow-core", replacement: wasmEngineStub },
    ],
  },
  test: {
    root: testRoot,
    name: "@semio-tech/framework-renderer-react",
    environment: "jsdom",
    coverage: { include: ["../../🟦️.tsx"] },
    exclude: [...configDefaults.exclude, ...playwrightEngineTestSuites, ...rootPolicySelfTestSuites],
    include: includeAgentBridge ? [agentBridgeTestSuite] : testLevel === "fundamental" || testLevel === "quick" ? [quickTestSuite] : [...engineTestSuites],
    testNamePattern: testLevel === "fundamental" ? /validates the language-neutral renderer resident capacity with the Node oracle/ : undefined,
    // 🧪️ In-source (`import.meta.vitest`) suites in the `🧑‍🎨engine/🧱️elements/` co-location dirs —
    // NOT under this package's own `root`, so the default `include` glob never finds them. Fundamental
    // and quick deliberately select the bounded resident-composition file; long restores the default
    // package corpus plus moderate in-source suites; exhaustive adds the expensive incremental ownership
    // matrices. A file must never appear in both `include` and `includeSource`, which would double-count it.
    includeSource: includeAgentBridge ? [] : includeBackboneWorker ? [backboneWorkerSuite] : testLevel === "exhaustive" ? [...exhaustiveInSourceSuites] : testLevel === "long" ? [...longInSourceSuites] : [],
  },
});

```

## Root Correction From The Actual Third Census

The extraction initially rebound simple literal includes but did not separately resolve two manifest-based concat/env includes. A documentation-prefix replacement also changed those two string fragments, retaining the old product directory before the new neutral path. That was Root's error, not a concurrent author delta. Third actual source census refused both missing targets; both actual suffixes now retain their original manifest-relative parent count and directly name the lower neutral owner. The original assertion bodies remain intact. Every current consumer is being verified with the canonical source resolver before another complete source run.

```json
[
  {
    "path": "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
    "beforeSha256": "b693ed79a4903bc1933d668d93bcdc0bd286b4700db9056c999a59a4c0b0ad2c",
    "afterSha256": "5bd5b3ade3701dc07ec8b0423e2a5d8ed2f142332c1eb69902ab633495bb7ff2",
    "exactBadRoute": "🧰️framework/🛍️products/💻️os/🔨️modules/🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json",
    "exactCorrectRoute": "🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json"
  },
  {
    "path": "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
    "beforeSha256": "aaa87b5cfb0a5afcd00adac783934fbca66666bf3e7bdbdccd3aa66fa9997516",
    "afterSha256": "d525a76fbe0683f73d4a1e206f43d91e4be791fd6ba83c0ea834aa764fca41f1",
    "exactBadRoute": "🧰️framework/🛍️products/💻️os/🔨️modules/🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json",
    "exactCorrectRoute": "🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json"
  }
]
```

Renderer correct original long-level four-law file is now runtime GREEN4/4/0skip in Vitest23.54s/Nx27.2s after actual NoTests registration RED3.4s. Exactly one existing suite was added to the original long/exhaustive roster; all other roster/config source is preserved. This bounded runtime proves actual React journal and flow argument consumers against all original cases; it does not substitute for the full long renderer corpus.

## All Thirteen Actual Consumer Target Resolutions

```json
[
  {
    "source": "🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs",
    "manifestPaths": [],
    "target": "🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json",
    "base": "source"
  },
  {
    "source": "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs",
    "manifestPaths": [],
    "target": "🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json",
    "base": "source"
  },
  {
    "source": "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs",
    "manifestPaths": [],
    "target": "🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json",
    "base": "source"
  },
  {
    "source": "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️node-graph/🧪️tests/🔬️unit/🦀️.rs",
    "manifestPaths": [],
    "target": "🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json",
    "base": "source"
  },
  {
    "source": "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
    "manifestPaths": [],
    "target": "🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json",
    "base": "source"
  },
  {
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🧪️node-graph-edit-rows/🦀️.rs",
    "manifestPaths": [],
    "target": "🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json",
    "base": "source"
  },
  {
    "source": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️node-graph-delete-row/🦀️.rs",
    "manifestPaths": [],
    "target": "🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json",
    "base": "source"
  },
  {
    "source": "🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🔬️node-graph-edit-rows/🦀️.rs",
    "manifestPaths": [],
    "target": "🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json",
    "base": "source"
  },
  {
    "source": "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
    "manifestPaths": [],
    "target": "🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json",
    "base": "source"
  },
  {
    "source": "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
    "manifestPaths": [
      "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml"
    ],
    "target": "🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json",
    "base": "manifest"
  },
  {
    "source": "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
    "manifestPaths": [
      "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml"
    ],
    "target": "🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json",
    "base": "manifest"
  },
  {
    "source": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs",
    "manifestPaths": [],
    "target": "🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json",
    "base": "source"
  },
  {
    "source": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs",
    "manifestPaths": [],
    "target": "🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json",
    "base": "source"
  }
]
```

All13 original Rust consumer includes now resolve to the one physical neutral fixture. For both actual manifest-based WFC expressions, the authored Cargo lib root was checked before using its manifest provenance. This is a bounded exact-target check; fresh complete context census/runtime still required.

## Original Complete Native Receipt

Sole Native executed the complete original ToolMachine registered long route: Nextest31 tests/one binary,31pass/0skip/.091s; compile12.01s/Nx18.0s. Actual binary694e6286e81486b9 dep-info pins nine current inputs, including the neutral row unit and lower fixture, with independently checked byte lengths and BLAKE3. This is actual current Rust row decoding and full original transaction/scrub/typing coverage. The exact native/source table is in the sole Native main execution report and tool-machine-current-compiled-source-origins.json. No product deletion or all higher consumer runtime claim follows.

After that terminal, Root integrates the two new language-neutral schema/corpus laws into the ordinary TestScript beside the entire existing TypeScript conformance file. The original native invocation is unchanged. The focused ownership command also explicitly rejects unknown arguments. Sole Native will verify the current normal full route including the new source laws before a combined current pass is claimed.
