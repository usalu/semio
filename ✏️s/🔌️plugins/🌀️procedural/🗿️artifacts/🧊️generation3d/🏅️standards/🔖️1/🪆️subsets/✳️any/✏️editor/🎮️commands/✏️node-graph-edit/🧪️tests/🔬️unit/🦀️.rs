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
fn one_live_wire(fixture: &FlowFixture) -> (String, String, String, String, String) {
    let synapse = fixture.synapses.first().expect("every bundled example wires at least one synapse");
    (synapse.id.clone(), synapse.from.clone(), synapse.from_port.clone(), synapse.to.clone(), synapse.to_port.clone())
}

fn wire_exists(fixture: &FlowFixture, from: &str, from_port: &str, to: &str, to_port: &str) -> bool {
    fixture.synapses.iter().any(|synapse| synapse.from == from && synapse.from_port == from_port && synapse.to == to && synapse.to_port == to_port)
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
        let (id, from, from_port, to, to_port) = one_live_wire(&read.fixture);
        (read.fixture.synapses.len(), id, from, from_port, to, to_port)
    };
    eprintln!("[DEBUG] wire under test {synapse_id}: {from}:{from_port} -> {to}:{to_port} of {before}");

    edit(&mut app, serde_json::json!([{ "operation": "disconnect", "synapseId": synapse_id }])).await;
    {
        let read = snapshot(&app);
        assert_eq!(read.fixture.synapses.len(), before - 1, "disconnect must remove exactly the one wire it names");
        assert!(!wire_exists(&read.fixture, &from, &from_port, &to, &to_port), "the cut wire must be gone: {synapse_id}");
    }

    edit(&mut app, serde_json::json!([{ "operation": "connect", "sourceNodeId": from, "sourcePortId": from_port, "targetNodeId": to, "targetPortId": to_port }])).await;
    {
        let read = snapshot(&app);
        assert_eq!(read.fixture.synapses.len(), before, "connect must restore exactly one wire");
        assert!(wire_exists(&read.fixture, &from, &from_port, &to, &to_port), "connect must re-wire {from}:{from_port} -> {to}:{to_port}");
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
        crate::widget_id(read.fixture.widgets.first().expect("the fixture must have a widget")).to_string()
    };

    edit(&mut app, serde_json::json!([{ "operation": "move", "nodeId": node_id, "x": -321.0, "y": 654.0 }])).await;

    let landed = {
        let read = snapshot(&app);
        let position = read.fixture.layout.get(node_id.as_str()).unwrap_or_else(|| panic!("widget {node_id} must have a pinned layout after a move"));
        (position.x, position.y)
    };
    assert_eq!(landed, (-321.0, 654.0), "move must place the widget exactly where the drag released it");
    eprintln!("[DEBUG] moved widget={node_id} to {landed:?}");
}
