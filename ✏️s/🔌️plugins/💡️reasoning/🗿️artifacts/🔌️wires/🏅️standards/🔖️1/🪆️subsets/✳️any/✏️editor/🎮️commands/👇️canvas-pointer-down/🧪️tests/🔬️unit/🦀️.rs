use super::*;
use crate::editor::wires::commands::{add_node, canvas_pointer_move, canvas_pointer_up};
use crate::editor::wires::unit_tests::context::{dispatch, new_app};
use crate::editor::wires::WiresCommand;
use crate::standards::v1::subsets::any::schema::inferences::find_board_node;
use semio_framework::kernel::Effect;
use semio_framework_plugin::{artifact_app_laws, PluginApp, INTERACTION_SELECT_ACTION_ID};

#[semio_framework_async_macros::async_test]
async fn pointer_drag_translates_node_by_screen_delta() {
    let mut app = new_app().await;
    dispatch(&mut app, WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() })).await;
    dispatch(&mut app, WiresCommand::CanvasPointerDown(CanvasPointerDown { id: Some("node-1".into()), x: 100.0, y: 100.0 })).await;
    dispatch(&mut app, WiresCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 140.0, y: 130.0 })).await;
    let node = find_board_node(&app.snapshot().expect("snapshot"), "node-1").expect("node-1").clone();
    assert_eq!(node.get("x").and_then(|value| value.as_f64()), Some(40.0));
    assert_eq!(node.get("y").and_then(|value| value.as_f64()), Some(30.0));
    dispatch(&mut app, WiresCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {})).await;
    // A coalesced drag collapses to a single undo step restoring the origin.
    app.handle_action("undo", None, &artifact_app_laws::meta("local")).await.expect("undo");
    let node = find_board_node(&app.snapshot().expect("snapshot"), "node-1").expect("node-1").clone();
    assert_eq!(node.get("x").and_then(|value| value.as_f64()), Some(0.0));
}

/// 🕹️ A hit requests `interactionSelect` for the "graph" domain's "node" granularity instead of
/// mutating config directly.
#[semio_framework_async_macros::async_test]
async fn pointer_down_requests_a_select_effect_for_the_hit_node() {
    let mut app = new_app().await;
    dispatch(&mut app, WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() })).await;
    let result = dispatch(&mut app, WiresCommand::CanvasPointerDown(CanvasPointerDown { id: Some("node-1".into()), x: 10.0, y: 20.0 })).await;
    let effect = result.requested_effects.iter().find(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == INTERACTION_SELECT_ACTION_ID)).expect("interactionSelect effect");
    let Effect::DispatchAction { args, .. } = effect else { unreachable!() };
    let args = args.clone().map(store::pack_rt::dsl_value_to_json).expect("select args");
    assert_eq!(args["domainId"], "graph");
    assert_eq!(args["merge"], "replace");
    assert!(args["targets"].as_str().expect("targets json").contains("node-1"));
}

#[semio_framework_async_macros::async_test]
async fn pointer_down_on_empty_space_requests_no_select_effect() {
    let mut app = new_app().await;
    let result = dispatch(&mut app, WiresCommand::CanvasPointerDown(CanvasPointerDown { id: None, x: 0.0, y: 0.0 })).await;
    assert!(result.requested_effects.is_empty());
    assert!(result.mutations.is_empty());
}
