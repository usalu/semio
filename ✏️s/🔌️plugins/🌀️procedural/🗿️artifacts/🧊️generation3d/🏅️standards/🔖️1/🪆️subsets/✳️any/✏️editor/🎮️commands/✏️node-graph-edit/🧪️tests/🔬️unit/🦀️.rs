use super::*;
use crate::app_fixture::{app, dispatch, snapshot, Generation3dApp};
use crate::editor::generation3d::Generation3dCommand;

const SLIDER_GESTURE_FIXTURE_JSON: &str = include_str!("../../../../../🧫️fixtures/🎚️slider-gesture.json");
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct SliderGestureFixture { schema: String, coalescing: Vec<SliderGestureRow>, ui_scope: SliderGestureUiScope }
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct SliderGestureRow { row: String, dispatches: Vec<SliderGestureDispatch>, coalesce_keys: Vec<Option<String>>, final_value: f64, undoable_edits: usize }
#[derive(serde::Deserialize)]
struct SliderGestureDispatch { operations: serde_json::Value }
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct SliderGestureUiScope { window_bodies: Vec<String>, panel_bodies: Vec<String>, utilities: bool, tools: bool, engagements: bool, measures: bool, labels: bool }


async fn edit(app: &mut Generation3dApp, operations: serde_json::Value) {
    dispatch(app, Generation3dCommand::NodeGraphEdit(NodeGraphEdit { operations_json: operations.to_string() })).await;
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

#[semio_framework_async_macros::async_test]
async fn every_slider_gesture_row_folds_one_press_into_one_edit_on_the_released_value() {
    let _serial = crate::test_serial::lock();
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
    }
}

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
