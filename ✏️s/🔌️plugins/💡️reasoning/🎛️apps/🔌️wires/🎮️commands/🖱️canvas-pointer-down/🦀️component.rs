//! 🖱️ 🖱️ Wires play app commands command — `canvas-pointer-down`.

use crate::apps::wires::config::{WiresConfig, WiresConfigMutation};
use crate::artifacts::wires::schema::{fixture_camera, node_position};
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::artifacts::wires::op::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "pointer-down")]
pub struct CanvasPointerDown {
    pub id: Option<String>,
    pub x: f64,
    pub y: f64,
}

pub fn handle(payload: &CanvasPointerDown, doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
    let document = doc.snapshot;
    match payload.id.as_deref().filter(|id| find_board_node(document, id).is_some()) {
        Some(id) => Ok(Emit::config(vec![WiresConfigMutation::SetSelection { ids: vec![id.to_string()] }, WiresConfigMutation::SetDrag { node_id: Some(id.to_string()), last_x: payload.x, last_y: payload.y }])),
        None => Ok(Emit::default()),
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::wires::commands::add_node;
    use crate::apps::wires::testkit::{dispatch, new_app};
    use crate::apps::wires::WiresCommand;
    use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
    use semio_framework_plugin::{testkit, PluginApp};

    #[test]
    fn pointer_drag_translates_node_by_screen_delta() {
        let mut app = new_app();
        dispatch(&mut app, WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() }));
        dispatch(&mut app, WiresCommand::CanvasPointerDown(CanvasPointerDown { id: Some("node-1".into()), x: 100.0, y: 100.0 }));
        dispatch(&mut app, WiresCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 140.0, y: 130.0 }));
        let node = find_board_node(&app.snapshot().expect("snapshot"), "node-1").expect("node-1").clone();
        assert_eq!(node.get("x").and_then(|value| value.as_f64()), Some(40.0));
        assert_eq!(node.get("y").and_then(|value| value.as_f64()), Some(30.0));
        dispatch(&mut app, WiresCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {}));
        // A coalesced drag collapses to a single undo step restoring the origin.
        app.handle_action("undo", None, &testkit::meta("local")).expect("undo");
        let node = find_board_node(&app.snapshot().expect("snapshot"), "node-1").expect("node-1").clone();
        assert_eq!(node.get("x").and_then(|value| value.as_f64()), Some(0.0));
    }
}
//#endregion 🧪️Tests
