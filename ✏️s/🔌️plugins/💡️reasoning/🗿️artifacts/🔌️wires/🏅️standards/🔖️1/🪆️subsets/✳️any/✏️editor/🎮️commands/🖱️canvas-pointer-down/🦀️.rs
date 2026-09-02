//! 🖱️ 🖱️ Wires play app commands command — `canvas-pointer-down`.

use crate::artifacts::wires::op::WiresMutation;
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::artifacts::wires::WiresSnapshot;
use crate::editor::wires::config::{WiresConfig, WiresConfigMutation};
use crate::editor::wires::{wires_select_effect, WIRES_GRANULARITY_NODE};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "pointer-down")]
pub struct CanvasPointerDown {
    pub id: Option<String>,
    pub x: f64,
    pub y: f64,
}

/// 🕹️ Selection is framework-owned now (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
/// a hit no longer writes `WiresConfigMutation::SetSelection` directly, it asks the host to
/// redispatch `interactionSelect` for the "graph" domain's "node" granularity — the in-flight drag
/// state (`SetDrag`) stays a plain config mutation since it is genuinely app-specific.
pub async fn handle(payload: &CanvasPointerDown, doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, WiresConfig>) -> Result<Emit<WiresMutation, WiresConfigMutation>, Fault> {
    let document = doc.snapshot;
    match payload.id.as_deref().filter(|id| find_board_node(document, id).is_some()) {
        Some(id) => Ok(Emit {
            config_mutations: vec![WiresConfigMutation::SetDrag { node_id: Some(id.to_string()), last_x: payload.x, last_y: payload.y }],
            effects: vec![wires_select_effect(&[id.to_string()], WIRES_GRANULARITY_NODE, "replace")],
            ..Default::default()
        }),
        None => Ok(Emit::default()),
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
    use crate::editor::wires::commands::{add_node, canvas_pointer_move, canvas_pointer_up};
    use crate::editor::wires::testkit::{dispatch, new_app};
    use crate::editor::wires::WiresCommand;
    use semio_framework::kernel::Effect;
    use semio_framework_plugin::{testkit, PluginApp, INTERACTION_SELECT_ACTION_ID};

    #[semio_framework_async_macros::async_test]
    async fn pointer_drag_translates_node_by_screen_delta() {
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

    /// 🕹️ A hit requests `interactionSelect` for the "graph" domain's "node" granularity instead of
    /// mutating config directly.
    #[semio_framework_async_macros::async_test]
    async fn pointer_down_requests_a_select_effect_for_the_hit_node() {
        let mut app = new_app();
        dispatch(&mut app, WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() }));
        let result = dispatch(&mut app, WiresCommand::CanvasPointerDown(CanvasPointerDown { id: Some("node-1".into()), x: 10.0, y: 20.0 }));
        let effect = result.requested_effects.iter().find(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == INTERACTION_SELECT_ACTION_ID)).expect("interactionSelect effect");
        let Effect::DispatchAction { args, .. } = effect else { unreachable!() };
        let args = args.clone().map(store::pack_rt::dsl_value_to_json).expect("select args");
        assert_eq!(args["domainId"], "graph");
        assert_eq!(args["merge"], "replace");
        assert!(args["targets"].as_str().expect("targets json").contains("node-1"));
    }

    #[semio_framework_async_macros::async_test]
    async fn pointer_down_on_empty_space_requests_no_select_effect() {
        let mut app = new_app();
        let result = dispatch(&mut app, WiresCommand::CanvasPointerDown(CanvasPointerDown { id: None, x: 0.0, y: 0.0 }));
        assert!(result.requested_effects.is_empty());
        assert!(result.mutations.is_empty());
    }
}
//#endregion 🧪️Tests
