//! 🖱️ Wires play app commands — canvas pointer interactions (down/move/up) that drive node dragging.

use crate::apps::wires::config::{WiresConfig, WiresConfigMutation};
use crate::artifacts::wires::engine::{fixture_camera, find_board_node, node_position};
use crate::artifacts::wires::op::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;
use dsl::DslValue;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️CanvasPointerDown
pub mod canvas_pointer_down {
    use super::*;

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
}
//#endregion 🔖️CanvasPointerDown

//#region 🔖️CanvasPointerMove
pub mod canvas_pointer_move {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "pointer-move")]
    pub struct CanvasPointerMove {
        pub x: f64,
        pub y: f64,
    }

    pub fn handle(payload: &CanvasPointerMove, doc: &ArtifactView<'_, WiresSnapshot>, cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
        let document = doc.snapshot;
        let config = cfg.snapshot;
        let Some(drag_node_id) = config.drag_node_id.clone() else { return Ok(Emit::default()) };
        let Some(node) = find_board_node(document, &drag_node_id) else { return Ok(Emit::default()) };
        let zoom = fixture_camera(&document.board_fixture).2.max(1e-6);
        let (cur_x, cur_y) = node_position(node);
        let (dx, dy) = ((payload.x - config.drag_last_x) / zoom, (payload.y - config.drag_last_y) / zoom);
        let mut patch = BTreeMap::new();
        patch.insert("x".into(), dsl::to_dsl_value(&(cur_x + dx)).unwrap_or(DslValue::Null));
        patch.insert("y".into(), dsl::to_dsl_value(&(cur_y + dy)).unwrap_or(DslValue::Null));
        Ok(Emit {
            artifact_mutations: vec![WiresMutation::PatchNode { node_id: drag_node_id.clone(), patch }],
            config_mutations: vec![WiresConfigMutation::SetDrag { node_id: Some(drag_node_id.clone()), last_x: payload.x, last_y: payload.y }],
            coalesce_key: Some(format!("drag:{drag_node_id}")),
            ..Default::default()
        })
    }
}
//#endregion 🔖️CanvasPointerMove

//#region 🔖️CanvasPointerUp
pub mod canvas_pointer_up {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "pointer-up")]
    pub struct CanvasPointerUp {}

    pub fn handle(_payload: &CanvasPointerUp, _doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
        Ok(Emit::config(vec![WiresConfigMutation::SetDrag { node_id: None, last_x: 0.0, last_y: 0.0 }]))
    }
}
//#endregion 🔖️CanvasPointerUp

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::wires::commands::node::add_node;
    use crate::apps::wires::testkit::{dispatch, new_app};
    use crate::apps::wires::WiresCommand;
    use crate::artifacts::wires::engine::find_board_node;
    use semio_framework_plugin::{testkit, PluginApp};

    #[test]
    fn pointer_drag_translates_node_by_screen_delta() {
        let mut app = new_app();
        dispatch(&mut app, WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() }));
        dispatch(&mut app, WiresCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { id: Some("node-1".into()), x: 100.0, y: 100.0 }));
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
