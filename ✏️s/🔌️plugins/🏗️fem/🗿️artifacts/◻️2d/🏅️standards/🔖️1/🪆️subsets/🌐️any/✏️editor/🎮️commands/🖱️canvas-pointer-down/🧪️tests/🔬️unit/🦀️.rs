use super::*;
use crate::editor::fem2d::commands::{canvas_pointer_move, canvas_pointer_up, set_active_example};
use crate::editor::fem2d::interaction::{fem2d_layer_to_canvas, FEM2D_INTERACTION_DOMAIN};
use crate::editor::fem2d::modes::edit::windows::model::{find_node_2d, screen_2d};
use crate::editor::fem2d::unit_tests::context::{dispatch, fem2d_app, Fem2dApp};
use crate::editor::fem2d::Fem2dCommand;
use crate::Viewport2d;
use semio_framework_plugin::Effect;

const CANVAS_WIDTH: f64 = 800.0;
const CANVAS_HEIGHT: f64 = 600.0;

async fn demo_app() -> Fem2dApp {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::examples::demo::ID.into() })).await;
    app
}

fn node_pixel(app: &mut Fem2dApp, id: &str) -> (f64, f64) {
    let snapshot = app.snapshot().expect("snapshot");
    let node = find_node_2d(&snapshot.nodes, id).expect("demo node").clone();
    fem2d_layer_to_canvas(&Viewport2d::default(), screen_2d(node.x, node.y), CANVAS_WIDTH, CANVAS_HEIGHT)
}

fn replay(result: &semio_framework_plugin::InvocationResult) -> (String, dsl::DslValue) {
    let effect = result.requested_effects.iter().find(|effect| matches!(effect, Effect::ReplayShellCommand { .. })).expect("a pointer command requests a shell replay");
    let Effect::ReplayShellCommand { action_id, args } = effect else { unreachable!() };
    (action_id.clone(), args.clone().expect("replay args"))
}

#[semio_framework_async_macros::async_test]
async fn canvas_pointer_down_on_a_node_requests_interaction_select() {
    let mut app = demo_app().await;
    let (x, y) = node_pixel(&mut app, "n1");
    let result = dispatch(&mut app, Fem2dCommand::CanvasPointerDown(CanvasPointerDown { x, y, width: CANVAS_WIDTH, height: CANVAS_HEIGHT, button: 0, shift: false, ctrl: false, meta: false, alt: false })).await;
    assert!(result.mutations.is_empty(), "a pick never mutates the document");
    let (action_id, args) = replay(&result);
    assert_eq!(action_id, semio_framework::INTERACTION_SELECT_ACTION_ID);
    assert_eq!(args.get("domainId").and_then(dsl::DslValue::as_str), Some(FEM2D_INTERACTION_DOMAIN));
    assert_eq!(args.get("merge").and_then(dsl::DslValue::as_str), Some("replace"));
    assert_eq!(args.get("method").and_then(dsl::DslValue::as_str), Some("pick"));
    let targets = args.get("targets").and_then(dsl::DslValue::as_str).expect("targets");
    assert!(targets.contains("\"id\":\"n1\"") && targets.contains("\"granularity\":\"node\""), "{targets}");
}

#[semio_framework_async_macros::async_test]
async fn canvas_pointer_down_modifiers_choose_the_merge_mode() {
    let mut app = demo_app().await;
    let (x, y) = node_pixel(&mut app, "n1");
    for (shift, ctrl, meta, merge) in [(true, false, false, "additive"), (false, true, false, "subtractive"), (false, false, true, "subtractive"), (true, true, false, "invertive")] {
        let result = dispatch(&mut app, Fem2dCommand::CanvasPointerDown(CanvasPointerDown { x, y, width: CANVAS_WIDTH, height: CANVAS_HEIGHT, button: 0, shift, ctrl, meta, alt: false })).await;
        assert_eq!(replay(&result).1.get("merge").and_then(dsl::DslValue::as_str), Some(merge), "shift={shift} ctrl={ctrl} meta={meta}");
    }
}

#[semio_framework_async_macros::async_test]
async fn canvas_pointer_down_on_empty_space_clears_the_selection() {
    let mut app = demo_app().await;
    let result = dispatch(&mut app, Fem2dCommand::CanvasPointerDown(CanvasPointerDown { x: CANVAS_WIDTH - 4.0, y: 4.0, width: CANVAS_WIDTH, height: CANVAS_HEIGHT, button: 0, shift: true, ctrl: false, meta: false, alt: false })).await;
    let (_, args) = replay(&result);
    assert_eq!(args.get("targets").and_then(dsl::DslValue::as_str), Some("[]"));
    assert_eq!(args.get("merge").and_then(dsl::DslValue::as_str), Some("replace"), "a background click always replaces, whatever the modifiers");
}

#[semio_framework_async_macros::async_test]
async fn canvas_pointer_down_ignores_non_primary_buttons() {
    let mut app = demo_app().await;
    let (x, y) = node_pixel(&mut app, "n1");
    let result = dispatch(&mut app, Fem2dCommand::CanvasPointerDown(CanvasPointerDown { x, y, width: CANVAS_WIDTH, height: CANVAS_HEIGHT, button: 2, shift: false, ctrl: false, meta: false, alt: false })).await;
    assert!(result.requested_effects.iter().all(|effect| !matches!(effect, Effect::ReplayShellCommand { .. })), "only button 0 picks");
}

#[semio_framework_async_macros::async_test]
async fn canvas_pointer_move_requests_hover_and_pointer_up_is_a_no_op() {
    let mut app = demo_app().await;
    let (x, y) = node_pixel(&mut app, "n2");
    let result = dispatch(&mut app, Fem2dCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x, y, width: CANVAS_WIDTH, height: CANVAS_HEIGHT })).await;
    let (action_id, args) = replay(&result);
    assert_eq!(action_id, semio_framework::INTERACTION_HOVER_ACTION_ID);
    assert_eq!(args.get("channel").and_then(dsl::DslValue::as_str), Some("pointer"));
    assert!(args.get("targets").and_then(dsl::DslValue::as_str).is_some_and(|raw| raw.contains("\"id\":\"n2\"")));

    let result = dispatch(&mut app, Fem2dCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { x, y, width: CANVAS_WIDTH, height: CANVAS_HEIGHT, shift: false, ctrl: false, meta: false, alt: false })).await;
    assert!(result.mutations.is_empty());
    assert!(result.requested_effects.iter().all(|effect| !matches!(effect, Effect::ReplayShellCommand { .. })), "a release commits nothing");
}
