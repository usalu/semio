use super::*;
use crate::editor::fem2d::commands::{canvas_pointer_move, canvas_pointer_up};
use crate::editor::fem2d::interaction::{canvas_gesture, fem2d_layer_to_canvas, FEM2D_INTERACTION_DOMAIN, FEM2D_POINTER_CHANNEL};
use crate::editor::fem2d::modes::edit::windows::model as model_window;
use crate::editor::fem2d::modes::edit::windows::results as results_window;
use crate::Viewport2d;
use model_window::{find_node_2d, screen_2d};
use semio_framework::kernel::{Effect, UiDirtyScope};
use semio_framework_plugin::{HistoryView, ViewModel, ViewWindowInstance};
use store::ArtifactDsl;

const CANVAS_WIDTH: f64 = 800.0;
const CANVAS_HEIGHT: f64 = 600.0;

fn demo() -> Fem2dSnapshot {
    Fem2dSnapshot::parse_dsl(crate::editor::fem2d::FEM2D_EXAMPLE_DSL).expect("demo document parses")
}

fn addressed(kind: &str) -> ViewModel {
    ViewModel {
        window_id: Some("w".into()),
        window_instances: vec![ViewWindowInstance { id: "w".into(), window_kind_id: kind.into() }],
        active_utility_id: Some(canvas_gesture::FEM2D_UTILITY_SELECT_DIRECT.into()),
        ..Default::default()
    }
}

fn node_pixel(doc: &Fem2dSnapshot, id: &str) -> (f64, f64) {
    let node = find_node_2d(&doc.nodes, id).expect("demo node");
    fem2d_layer_to_canvas(&Viewport2d::default(), screen_2d(node.x, node.y), CANVAS_WIDTH, CANVAS_HEIGHT)
}

fn press(doc: &Fem2dSnapshot, kind: &str, x: f64, y: f64, button: u32, shift: bool, ctrl: bool, meta: bool) -> Emit<Fem2dMutation, NoConfigMutation> {
    let history = HistoryView::empty();
    let view = ArtifactView::new(doc, &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    handle_window(&CanvasPointerDown { x, y, width: CANVAS_WIDTH, height: CANVAS_HEIGHT, button, shift, ctrl, meta, alt: false }, &view, &cfg, &addressed(kind)).expect("pointer down")
}

fn replay(emit: &Emit<Fem2dMutation, NoConfigMutation>) -> (&str, &dsl::DslValue) {
    let Some(Effect::ReplayShellCommand { action_id, args }) = emit.effects.first() else { panic!("a pointer command requests exactly one shell replay") };
    (action_id.as_str(), args.as_ref().expect("replay args"))
}

#[semio_framework_async_macros::async_test]
async fn canvas_pointer_down_on_a_node_requests_interaction_select() {
    let doc = demo();
    let (x, y) = node_pixel(&doc, "n1");
    let emit = press(&doc, model_window::WINDOW_KIND_ID, x, y, 0, false, false, false);
    assert!(emit.artifact_mutations.is_empty() && emit.window_config_mutations.is_empty(), "a pick never writes a store lane");
    let (action_id, args) = replay(&emit);
    assert_eq!(action_id, semio_framework::INTERACTION_SELECT_ACTION_ID);
    assert_eq!(args.get("domainId").and_then(dsl::DslValue::as_str), Some(FEM2D_INTERACTION_DOMAIN));
    assert_eq!(args.get("merge").and_then(dsl::DslValue::as_str), Some("replace"));
    assert_eq!(args.get("method").and_then(dsl::DslValue::as_str), Some("pick"));
    let targets = args.get("targets").and_then(dsl::DslValue::as_str).expect("targets");
    assert!(targets.contains("\"id\":\"n1\"") && targets.contains("\"granularity\":\"node\""), "{targets}");
}

#[semio_framework_async_macros::async_test]
async fn canvas_pointer_down_modifiers_choose_the_merge_mode() {
    let doc = demo();
    let (x, y) = node_pixel(&doc, "n1");
    for (shift, ctrl, meta, merge) in [(true, false, false, "additive"), (false, true, false, "subtractive"), (false, false, true, "subtractive"), (true, true, false, "invertive")] {
        let emit = press(&doc, model_window::WINDOW_KIND_ID, x, y, 0, shift, ctrl, meta);
        assert_eq!(replay(&emit).1.get("merge").and_then(dsl::DslValue::as_str), Some(merge), "shift={shift} ctrl={ctrl} meta={meta}");
    }
}

#[semio_framework_async_macros::async_test]
async fn canvas_pointer_down_on_empty_space_clears_the_selection() {
    let doc = demo();
    let emit = press(&doc, model_window::WINDOW_KIND_ID, CANVAS_WIDTH - 4.0, 4.0, 0, true, false, false);
    let (_, args) = replay(&emit);
    assert_eq!(args.get("targets").and_then(dsl::DslValue::as_str), Some("[]"));
    assert_eq!(args.get("merge").and_then(dsl::DslValue::as_str), Some("replace"), "a background click always replaces, whatever the modifiers");
}

#[semio_framework_async_macros::async_test]
async fn canvas_pointer_down_ignores_non_primary_buttons() {
    let doc = demo();
    let (x, y) = node_pixel(&doc, "n1");
    assert!(press(&doc, model_window::WINDOW_KIND_ID, x, y, 2, false, false, false).effects.is_empty(), "only button 0 picks");
    assert!(press(&doc, model_window::WINDOW_KIND_ID, x, y, 1, false, false, false).effects.is_empty(), "the middle button pans in the host");
}

#[semio_framework_async_macros::async_test]
async fn the_results_window_picks_with_its_own_camera() {
    let doc = demo();
    let (x, y) = node_pixel(&doc, "p8");
    let emit = press(&doc, results_window::WINDOW_KIND_ID, x, y, 0, false, false, false);
    assert!(replay(&emit).1.get("targets").and_then(dsl::DslValue::as_str).is_some_and(|raw| raw.contains("\"id\":\"p8\"")));
}

#[semio_framework_async_macros::async_test]
async fn an_unaddressed_pointer_event_faults() {
    let doc = demo();
    let history = HistoryView::empty();
    let view = ArtifactView::new(&doc, &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let payload = CanvasPointerDown { x: 0.0, y: 0.0, width: CANVAS_WIDTH, height: CANVAS_HEIGHT, button: 0, shift: false, ctrl: false, meta: false, alt: false };
    assert!(handle_window(&payload, &view, &cfg, &ViewModel::default()).is_err(), "a pick without an addressed window has no camera to invert");
    assert!(handle(&payload, &view, &cfg).is_err(), "the doc-scoped route always refuses");
}

#[semio_framework_async_macros::async_test]
async fn canvas_pointer_move_requests_hover_and_pointer_up_refreshes_the_window() {
    let doc = demo();
    let history = HistoryView::empty();
    let view = ArtifactView::new(&doc, &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let (x, y) = node_pixel(&doc, "n2");
    let emit = canvas_pointer_move::handle_window(&canvas_pointer_move::CanvasPointerMove { x, y, width: CANVAS_WIDTH, height: CANVAS_HEIGHT, samples: Vec::new() }, &view, &cfg, &addressed(model_window::WINDOW_KIND_ID)).expect("hover");
    let (action_id, args) = replay(&emit);
    assert_eq!(action_id, semio_framework::INTERACTION_HOVER_ACTION_ID);
    assert_eq!(args.get("channel").and_then(dsl::DslValue::as_str), Some(FEM2D_POINTER_CHANNEL));
    assert!(args.get("targets").and_then(dsl::DslValue::as_str).is_some_and(|raw| raw.contains("\"id\":\"n2\"")));

    let released = canvas_pointer_up::handle_window(
        &canvas_pointer_up::CanvasPointerUp { x, y, width: CANVAS_WIDTH, height: CANVAS_HEIGHT, shift: false, ctrl: false, meta: false, alt: false, cancelled: false },
        &view,
        &cfg,
        &addressed(model_window::WINDOW_KIND_ID),
    )
    .expect("release");
    assert!(matches!(released.ui_scope, UiDirtyScope::Partial { .. }), "a release clears any in-flight marquee overlay");
}
