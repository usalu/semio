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

/// 🧾️ Every applied history row that carries document operations.
async fn edit_rows(app: &mut Generation3dApp) -> Vec<HistoryEntry> {
    PluginApp::history_snapshot(app).await.expect("history").upserts.into_iter().filter(|entry| entry.applied && !entry.op_lines.is_empty()).collect()
}

/// 📍️ Places every widget of the document on the canvas in ONE setup edit, so a node drag has base positions; answers
/// the placed ids.
async fn place_every_widget(app: &mut Generation3dApp) -> Vec<String> {
    let mut placed = snapshot(app).host_snapshot.clone();
    let ids: Vec<String> = placed.widgets.iter().map(|widget| crate::widget_id(widget).to_string()).collect();
    for (index, id) in ids.iter().enumerate() {
        let _ = placed.layout.insert(id.clone(), semio_framework_artifact_flow_flow::WidgetLayout { x: index as f64 * 100.0, y: 50.0 });
    }
    let json = dsl::json::to_json_string(&dsl::ToValue::to_value(&placed));
    placed.retire_cold();
    dispatch(app, Generation3dCommand::NodeGraphEdit(NodeGraphEdit { operations_json: serde_json::json!([{ "operation": "setHostSnapshot", "hostSnapshotJson": json }]).to_string() })).await;
    ids
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
    let ids = place_every_widget(&mut app).await;
    let dragged = vec![ids[0].clone(), ids[1].clone()];
    let before = edit_rows(&mut app).await.len();
    send(&mut app, serde_json::json!({ "operations": [{ "operation": "move", "gestureId": "node-drag:1", "nodeIds": dragged, "dx": 40.0, "dy": -12.5 }] })).await;
    let read = snapshot(&app);
    assert_eq!(read.host_snapshot.layout.get(ids[0].as_str()).map(|layout| (layout.x, layout.y)), Some((40.0, 37.5)));
    assert_eq!(read.host_snapshot.layout.get(ids[1].as_str()).map(|layout| (layout.x, layout.y)), Some((140.0, 37.5)));
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
    let ids = place_every_widget(&mut app).await;
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
    assert_eq!(snapshot(&app).host_snapshot.layout.get(ids[0].as_str()).map(|layout| (layout.x, layout.y)), Some((10.0, 60.0)));
}
//#endregion ✋️NodeDrag
