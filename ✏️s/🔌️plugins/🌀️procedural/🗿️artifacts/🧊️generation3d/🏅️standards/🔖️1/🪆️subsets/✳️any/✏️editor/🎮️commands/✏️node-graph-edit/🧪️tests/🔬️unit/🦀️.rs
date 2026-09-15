use super::*;
use crate::editor::generation3d::unit_tests::context::{app, dispatch, snapshot, Generation3dApp};
use crate::editor::generation3d::Generation3dCommand;

async fn edit(app: &mut Generation3dApp, operations: serde_json::Value) {
    dispatch(app, Generation3dCommand::NodeGraphEdit(NodeGraphEdit { operations_json: operations.to_string() })).await;
}

/// 🔌️ One live wire of the current document, as the four endpoint coordinates a port-to-port drag
/// reports plus the synapse id a wire cut names: `(id, fromNode, fromPort, toNode, toPort)`. Read off
/// the document's OWN synapses rather than off a port projection, so the law needs no `FlowHost`
/// (building one inside a 2 MiB test thread overflows the stack).
fn one_live_wire(host_snapshot: &FlowHostSnapshot) -> (String, String, String, String, String) {
    let synapse = host_snapshot.synapses.first().expect("every bundled example wires at least one synapse");
    (synapse.id.clone(), synapse.from.clone(), synapse.from_port.clone(), synapse.to.clone(), synapse.to_port.clone())
}

fn wire_exists(host_snapshot: &FlowHostSnapshot, from: &str, from_port: &str, to: &str, to_port: &str) -> bool {
    host_snapshot.synapses.iter().any(|synapse| synapse.from == from && synapse.from_port == from_port && synapse.to == to && synapse.to_port == to_port)
}

/// ⚖️ LAW: `disconnect` cuts a wire and `connect` draws it again between the same two ports — the two
/// halves of node-graph wire editing, both reachable through the ONE `nodeGraphEdit` command the
/// canvas dispatches, and both asserted on the document's own synapse table.
///
/// 🐛️ `disconnect` used to fall through the `_ => {}` arm, so every wire cut was a silent no-op that
/// still spent a whole retained command (`📓️audit-user-journey-gaps-2026-09-13.md` gap #3).
#[semio_framework_async_macros::async_test]
async fn disconnect_then_connect_round_trips_one_wire() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    let (before, synapse_id, from, from_port, to, to_port) = {
        let read = snapshot(&app);
        let (id, from, from_port, to, to_port) = one_live_wire(&read.host_snapshot);
        (read.host_snapshot.synapses.len(), id, from, from_port, to, to_port)
    };
    eprintln!("[DEBUG] wire under test {synapse_id}: {from}:{from_port} -> {to}:{to_port} of {before}");

    edit(&mut app, serde_json::json!([{ "operation": "disconnect", "synapseId": synapse_id }])).await;
    {
        let read = snapshot(&app);
        assert_eq!(read.host_snapshot.synapses.len(), before - 1, "disconnect must remove exactly the one wire it names");
        assert!(!wire_exists(&read.host_snapshot, &from, &from_port, &to, &to_port), "the cut wire must be gone: {synapse_id}");
    }

    edit(&mut app, serde_json::json!([{ "operation": "connect", "sourceNodeId": from, "sourcePortId": from_port, "targetNodeId": to, "targetPortId": to_port }])).await;
    {
        let read = snapshot(&app);
        assert_eq!(read.host_snapshot.synapses.len(), before, "connect must restore exactly one wire");
        assert!(wire_exists(&read.host_snapshot, &from, &from_port, &to, &to_port), "connect must re-wire {from}:{from_port} -> {to}:{to_port}");
    }
}

/// ⚖️ LAW: `move` relocates a widget. It is the ONE spelling of the node-move verb — the duplicate
/// `moveMediaNode` command, which had no dispatcher anywhere in the renderer, was deleted with this
/// law (`📓️audit-user-journey-gaps-2026-09-13.md` gap #10) — and it is exactly what the `Diagram`
/// fallback's `onNodeDragStop` sends.
#[semio_framework_async_macros::async_test]
async fn move_relocates_the_widget_a_node_drag_names() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    let node_id = {
        let read = snapshot(&app);
        crate::widget_id(read.host_snapshot.widgets.first().expect("the fixture must have a widget")).to_string()
    };

    edit(&mut app, serde_json::json!([{ "operation": "move", "nodeId": node_id, "x": -321.0, "y": 654.0 }])).await;

    let landed = {
        let read = snapshot(&app);
        let position = read.host_snapshot.layout.get(node_id.as_str()).unwrap_or_else(|| panic!("widget {node_id} must have a pinned layout after a move"));
        (position.x, position.y)
    };
    assert_eq!(landed, (-321.0, 654.0), "move must place the widget exactly where the drag released it");
    eprintln!("[DEBUG] moved widget={node_id} to {landed:?}");
}

//#region 🎚️SliderGesture
/// 🎚️ The table a dragged inline slider must answer — the twin of the host lane's own
/// (`📺️renderer/🧑‍🎨engine/🧪️tests/🎚️continuous-gesture-lane`), over the same numbers.
const SLIDER_GESTURE_FIXTURE_JSON: &str = include_str!("../../../../../🧫️fixtures/🎚️slider-gesture.json");

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct SliderGestureFixture {
    schema: String,
    coalescing: Vec<SliderGestureRow>,
    ui_scope: SliderGestureUiScope,
    live_evaluations: SliderGestureLiveEvaluations,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct SliderGestureRow {
    row: String,
    dispatches: Vec<SliderGestureDispatch>,
    coalesce_keys: Vec<Option<String>>,
    final_value: f64,
    undoable_edits: usize,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct SliderGestureDispatch {
    operations: serde_json::Value,
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

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct SliderGestureLiveEvaluations {
    max_live_round_trips: usize,
    commits_per_press: usize,
}

fn slider_gesture_table() -> SliderGestureFixture {
    let table: SliderGestureFixture = serde_json::from_str(SLIDER_GESTURE_FIXTURE_JSON).expect("slider gesture fixture");
    assert_eq!(table.schema, "s.procedural.generation3d.slider-gesture/v1");
    table
}

fn slider_value(host_snapshot: &FlowHostSnapshot, widget_id: &str) -> Option<f64> {
    host_snapshot.widgets.iter().find_map(|widget| match widget {
        semio_framework_artifact_flow_flow::Widget::InputSlider { id, value, .. } if id == widget_id => Some(*value),
        _ => None,
    })
}

fn sub_operations_of(operations: &serde_json::Value) -> Vec<dsl::json::Value> {
    parse_sub_operations(&operations.to_string())
}

/// ⚖️ LAW: every row of `🎚️slider-gesture.json` folds ONE press into ONE undoable edit under that
/// press's own key, and the document ends on the value the user released on.
///
/// 🐛️ The overlay used to dispatch `setGraphParameter`, an action NO window kind declares. The shell
/// dropped every tick of every drag (`dropped action "setGraphParameter" … no window kind declares
/// it`, six per one-second drag on 6018), so the knob moved and the document did not — which is
/// exactly the "moving a slider doesn't update the preview" the user reported. Routing it through
/// `nodeGraphEdit` fixed reachability; without the coalesce key it then cost one history entry per
/// tick (`📓️slider-preview-update-2026-09-15.md`).
#[semio_framework_async_macros::async_test]
async fn every_slider_gesture_row_folds_one_press_into_one_edit_on_the_released_value() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let table = slider_gesture_table();
    for row in &table.coalescing {
        assert_eq!(row.dispatches.len(), row.coalesce_keys.len(), "row {} must state one expected key per dispatch", row.row);
        let mut app = app().await;
        let mut keys: Vec<Option<String>> = Vec::new();
        let mut distinct: Vec<String> = Vec::new();
        for dispatch_row in &row.dispatches {
            let operations = sub_operations_of(&dispatch_row.operations);
            let key = gesture_coalesce_key(&operations);
            if let Some(key) = key.clone() {
                if !distinct.contains(&key) {
                    distinct.push(key);
                }
            } else {
                distinct.push(format!("described:{}", distinct.len()));
            }
            keys.push(key);
            edit(&mut app, dispatch_row.operations.clone()).await;
        }
        assert_eq!(keys, row.coalesce_keys, "row {} must fold under exactly the keys it states", row.row);
        assert_eq!(distinct.len(), row.undoable_edits, "row {} must cost exactly {} undoable edit(s), got {distinct:?}", row.row, row.undoable_edits);
        let landed = {
            let read = snapshot(&app);
            slider_value(&read.host_snapshot, "height")
        };
        assert_eq!(landed, Some(row.final_value), "row {} must leave the document on the released value", row.row);
        eprintln!("[DEBUG] slider gesture row={} keys={keys:?} edits={} value={landed:?}", row.row, distinct.len());
    }
}

/// ⚖️ LAW: a live slider tick declares the NARROW refresh scope the table states, and a discrete
/// graph edit keeps the framework's full one.
///
/// 🐛️ Every tick of a drag re-rendered every window body, every panel body and all five rails.
#[semio_framework_async_macros::async_test]
async fn a_slider_tick_declares_the_narrow_scope_and_a_discrete_edit_keeps_full() {
    let table = slider_gesture_table();
    let scope = slider_gesture_ui_scope();
    match scope {
        UiDirtyScope::Partial { window_bodies, panel_bodies, utilities, tools, engagements, measures, labels } => {
            assert_eq!(window_bodies, table.ui_scope.window_bodies, "a slider tick repaints exactly the stated window bodies");
            assert_eq!(panel_bodies, table.ui_scope.panel_bodies, "a slider tick repaints exactly the stated panel bodies");
            assert_eq!((utilities, tools, engagements, measures, labels), (table.ui_scope.utilities, table.ui_scope.tools, table.ui_scope.engagements, table.ui_scope.measures, table.ui_scope.labels), "a slider tick touches no rail");
        }
        other => panic!("a slider tick must declare a partial scope, got {other:?}"),
    }
    let discrete = sub_operations_of(&serde_json::json!([{ "operation": "move", "nodeId": "height", "x": 1.0, "y": 2.0 }]));
    assert_eq!(gesture_coalesce_key(&discrete), None, "a discrete graph edit folds under no press");
}

/// ⚖️ LAW: N rapid values cost at most two live round trips and exactly one commit per press — the
/// host lane's ceiling, stated here over the SAME table the TypeScript twin reads so the two
/// implementations cannot disagree about what a gesture is allowed to cost.
#[semio_framework_async_macros::async_test]
async fn a_press_spends_at_most_two_live_round_trips_and_one_commit() {
    let table = slider_gesture_table();
    assert_eq!(table.live_evaluations.max_live_round_trips, 2, "one round trip in flight and one owed is the whole budget");
    assert_eq!(table.live_evaluations.commits_per_press, 1, "a press releases once");
    let press = slider_gesture_table()
        .coalescing
        .into_iter()
        .find(|row| row.row == "one-press-folds-into-one-edit")
        .expect("the table must carry the one-press row");
    let commits = press
        .dispatches
        .iter()
        .filter(|dispatch_row| {
            sub_operations_of(&dispatch_row.operations)
                .iter()
                .any(|operation| operation.get("commit").and_then(dsl::json::Value::as_bool) == Some(true))
        })
        .count();
    assert_eq!(commits, table.live_evaluations.commits_per_press, "exactly one dispatch of a press carries the release");
}
//#endregion 🎚️SliderGesture
