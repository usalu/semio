use super::*;
use crate::editor::generation2d::unit_tests::context::{app, close, dispatch, snapshot_read};
use crate::editor::generation2d::Generation2dCommand;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework::kernel::UiDirtyScope;

async fn edit(app: &mut crate::editor::generation2d::unit_tests::context::Generation2dApp, operations: serde_json::Value) {
    dispatch(app, Generation2dCommand::NodeGraphEdit(NodeGraphEdit { operations_json: operations.to_string() })).await;
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


/// 🧪️ Installs `math.add` with the ports the demo fixture wires (`a`/`sum`), so `FlowHost`
/// reconnect can resolve endpoints after a disconnect.
fn install_math_add_operator() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let manifest = semio_framework_os_flow::FlowExtensionManifest {
            schema: "flow.extension".into(),
            id: "math".into(),
            name: "Math".into(),
            version: "0.0.0-test-fixture".into(),
            activation_events: vec!["onStartup".into()],
            contributes: semio_framework_os_flow::FlowExtensionContributes {
                schemas: vec![],
                operators: vec![semio_framework_os_flow::neural::OperatorInfo {
                    id: "math.add".into(),
                    extension: "math".into(),
                    name: "Add".into(),
                    abbreviation: "Add".into(),
                    inputs: vec![semio_framework_os_flow::neural::ChannelSpec::any("a"), semio_framework_os_flow::neural::ChannelSpec::any("b")],
                    outputs: vec![semio_framework_os_flow::neural::ChannelSpec::any("sum")],
                    ..Default::default()
                }],
                widgets: vec![],
                commands: vec![],
                settings: vec![],
            },
        };
        let manifest_json = semio_framework_os_flow::os_pack::json::to_json_string(&manifest);
        semio_framework_os_flow::install_flow_extension_manifest("generation2d-node-graph-edit-math", &manifest_json).expect("fixture extension admission");
    });
}

/// ⚖️ LAW: `disconnect` cuts a wire and `connect` draws it again between the same two ports.
#[semio_framework_async_macros::async_test]
async fn disconnect_then_connect_round_trips_one_wire() {
    install_math_add_operator();
    let mut app = app().await;
    let (before, synapse_id, from, from_port, to, to_port) = {
        let read = snapshot_read(&app);
        let (id, from, from_port, to, to_port) = one_live_wire(&read.host_snapshot);
        (read.host_snapshot.synapses.len(), id, from, from_port, to, to_port)
    };
    eprintln!("[DEBUG] wire under test {synapse_id}: {from}:{from_port} -> {to}:{to_port} of {before}");

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

/// ⚖️ LAW: `move` relocates a widget exactly where the drag released it.
#[semio_framework_async_macros::async_test]
async fn move_relocates_the_widget_a_node_drag_names() {
    let mut app = app().await;
    let node_id = {
        let read = snapshot_read(&app);
        crate::widget_id(read.host_snapshot.widgets.first().expect("the fixture must have a widget")).to_string()
    };

    edit(&mut app, serde_json::json!([{ "operation": "move", "nodeId": node_id, "x": -321.0, "y": 654.0 }])).await;

    let landed = {
        let read = snapshot_read(&app);
        let position = read.host_snapshot.layout.get(node_id.as_str()).unwrap_or_else(|| panic!("widget {node_id} must have a pinned layout after a move"));
        (position.x, position.y)
    };
    assert_eq!(landed, (-321.0, 654.0), "move must place the widget exactly where the drag released it");
    eprintln!("[DEBUG] moved widget={node_id} to {landed:?}");
    close(app);
}

/// ⚖️ LAW: `setSlider` writes the live slider value onto the document.
#[semio_framework_async_macros::async_test]
async fn set_slider_writes_the_released_value() {
    let mut app = app().await;
    edit(&mut app, serde_json::json!([{ "operation": "setSlider", "widgetId": "slider", "value": 7.5, "gesture": "press-1", "commit": true }])).await;
    let landed = {
        let read = snapshot_read(&app);
        slider_value(&read.host_snapshot, "slider")
    };
    assert_eq!(landed, Some(7.5), "setSlider must leave the document on the released value");
    eprintln!("[DEBUG] slider value={landed:?}");
    close(app);
}

/// ⚖️ LAW: one continuous slider press folds under one coalesce key, and a discrete move folds under none.
#[test]
fn set_slider_gesture_coalesce_and_scope() {
    let press = parse_sub_operations(r#"[{"operation":"setSlider","widgetId":"slider","value":1.0,"gesture":"g1"},{"operation":"setSlider","widgetId":"slider","value":2.0,"gesture":"g1","commit":true}]"#);
    assert_eq!(gesture_coalesce_key(&press), Some("graph-slider:g1".into()));
    match slider_gesture_ui_scope() {
        UiDirtyScope::Partial { utilities, tools, engagements, measures, labels, .. } => {
            assert_eq!((utilities, tools, engagements, measures, labels), (false, false, false, false, false));
        }
        other => panic!("a slider tick must declare a partial scope, got {other:?}"),
    }
    let discrete = parse_sub_operations(r#"[{"operation":"move","nodeId":"slider","x":1.0,"y":2.0}]"#);
    assert_eq!(gesture_coalesce_key(&discrete), None);
}
