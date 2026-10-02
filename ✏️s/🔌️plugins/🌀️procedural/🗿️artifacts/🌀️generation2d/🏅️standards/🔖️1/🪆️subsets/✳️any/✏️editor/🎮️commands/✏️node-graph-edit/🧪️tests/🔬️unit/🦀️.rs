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
const NODE_GRAPH_EDIT_ROWS_JSON: &str = include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json");

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
