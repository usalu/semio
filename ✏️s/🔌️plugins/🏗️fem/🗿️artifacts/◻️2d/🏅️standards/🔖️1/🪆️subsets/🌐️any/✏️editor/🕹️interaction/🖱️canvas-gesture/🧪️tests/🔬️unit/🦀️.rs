use super::*;
use crate::editor::fem2d::interaction::{fem2d_layer_to_canvas, FEM2D_GRANULARITY_NODE};
use crate::editor::fem2d::modes::edit::windows::model::{find_node_2d, screen_2d, WINDOW_KIND_ID};
use crate::Viewport2d;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::{ArtifactView, ConfigView, HistoryView, NoConfig, ViewModel, ViewWindowInstance};
use store::ArtifactDsl;

type Fem2dSnapshot = crate::Fem2dSnapshot;

const CANVAS_WIDTH: f64 = 800.0;
const CANVAS_HEIGHT: f64 = 600.0;

fn demo() -> Fem2dSnapshot {
    Fem2dSnapshot::parse_dsl(crate::editor::fem2d::FEM2D_EXAMPLE_DSL).expect("demo document parses")
}

fn addressed(utility: &str) -> ViewModel {
    ViewModel {
        window_id: Some("w".into()),
        window_instances: vec![ViewWindowInstance { id: "w".into(), window_kind_id: WINDOW_KIND_ID.into() }],
        active_utility_id: Some(utility.into()),
        ..Default::default()
    }
}

#[test]
fn marquee_overlay_uses_canvas2d_path_segments() {
    let gesture = Fem2dCanvasGesture {
        tracking: true,
        active: true,
        method: "rectangle".to_string(),
        canvas_points: vec![(0.0, 0.0), (100.0, 80.0)],
        layer_points: vec![(0.0, 0.0), (100.0, 80.0)],
    };
    let layers = fem2d_marquee_overlay_layers(&gesture);
    assert_eq!(layers.len(), 1);
    let segments = layers[0].get("segments").and_then(|value| value.as_array()).expect("segments array");
    assert!(segments.iter().all(|segment| segment.get("kind").is_some()));
    assert_eq!(layers[0].get("role").and_then(|value| value.as_str()), Some("overlay"));
}

#[test]
fn marquee_hits_multiple_nodes_in_a_screen_rectangle() {
    let doc = demo();
    let camera = Viewport2d::default();
    let n1 = find_node_2d(&doc.nodes, "n1").expect("n1");
    let n2 = find_node_2d(&doc.nodes, "n2").expect("n2");
    let p1 = fem2d_layer_to_canvas(&camera, screen_2d(n1.x, n1.y), CANVAS_WIDTH, CANVAS_HEIGHT);
    let p2 = fem2d_layer_to_canvas(&camera, screen_2d(n2.x, n2.y), CANVAS_WIDTH, CANVAS_HEIGHT);
    let min_x = p1.0.min(p2.0) - 20.0;
    let max_x = p1.0.max(p2.0) + 20.0;
    let min_y = p1.1.min(p2.1) - 20.0;
    let max_y = p1.1.max(p2.1) + 20.0;
    let hits = fem2d_marquee_hits(&doc, &camera, "rectangle", &[(min_x, min_y), (max_x, max_y)], CANVAS_WIDTH, CANVAS_HEIGHT);
    assert!(hits.iter().any(|(granularity, id)| *granularity == FEM2D_GRANULARITY_NODE && id == "n1"));
    assert!(hits.iter().any(|(granularity, id)| *granularity == FEM2D_GRANULARITY_NODE && id == "n2"));
}

#[semio_framework_async_macros::async_test]
async fn marquee_drag_commits_on_pointer_up() {
    let doc = demo();
    let history = HistoryView::empty();
    let view = ArtifactView::new(&doc, &history);
    let cfg = ConfigView { snapshot: &NoConfig::default(), window: None };
    let model = addressed(FEM2D_UTILITY_SELECT_MARQUEE);
    let camera = Viewport2d::default();
    let start = (12.0, 12.0);
    let end = (CANVAS_WIDTH - 12.0, CANVAS_HEIGHT - 12.0);
    pointer_down(view.snapshot, &camera, &model, start.0, start.1, CANVAS_WIDTH, CANVAS_HEIGHT, 0, "replace").expect("down");
    pointer_move(view.snapshot, &camera, &model, &[end], CANVAS_WIDTH, CANVAS_HEIGHT).expect("move");
    let emit = pointer_up(view.snapshot, &camera, &model, end.0, end.1, CANVAS_WIDTH, CANVAS_HEIGHT, "replace", false).expect("up");
    let Some(semio_framework::kernel::Effect::ReplayShellCommand { args, .. }) = emit.effects.first() else { panic!("marquee commits through interactionSelect") };
    let args = args.as_ref().expect("args");
    assert_eq!(args.get("method").and_then(dsl::DslValue::as_str), Some("rectangle"));
    assert!(args.get("targets").and_then(dsl::DslValue::as_str).is_some_and(|raw| raw.contains("n1") && raw.contains("n2")));
    let _ = (&view, &cfg);
}

//#region 🧪️BatchedSamples
const LASSO_PATH: [(f64, f64); 4] = [(40.0, 40.0), (400.0, 40.0), (400.0, 500.0), (40.0, 500.0)];

#[test]
fn a_batch_of_samples_builds_the_same_lasso_polyline_as_separate_moves() {
    let doc = demo();
    let camera = Viewport2d::default();
    let start = (12.0, 12.0);

    let separate = addressed(FEM2D_UTILITY_SELECT_LASSO);
    pointer_down(&doc, &camera, &separate, start.0, start.1, CANVAS_WIDTH, CANVAS_HEIGHT, 0, "replace").expect("down");
    for sample in LASSO_PATH {
        pointer_move(&doc, &camera, &separate, &[sample], CANVAS_WIDTH, CANVAS_HEIGHT).expect("move");
    }
    let one_per_event = gesture_for_window("w");
    clear_gesture("w");

    let batched = addressed(FEM2D_UTILITY_SELECT_LASSO);
    pointer_down(&doc, &camera, &batched, start.0, start.1, CANVAS_WIDTH, CANVAS_HEIGHT, 0, "replace").expect("down");
    let emit = pointer_move(&doc, &camera, &batched, &LASSO_PATH, CANVAS_WIDTH, CANVAS_HEIGHT).expect("batched move");
    let one_batch = gesture_for_window("w");
    clear_gesture("w");

    assert_eq!(one_batch, one_per_event, "four samples in one command trace the polyline four moves did");
    assert_eq!(one_batch.canvas_points.len(), 1 + LASSO_PATH.len(), "every sample joins the path");
    assert!(one_batch.active, "the threshold is judged against the last sample");
    assert!(matches!(emit.ui_scope, UiDirtyScope::Partial { .. }), "one refresh per batch");
    assert!(emit.effects.is_empty(), "a tracking move never hovers");
}

#[test]
fn a_rectangle_batch_reads_its_last_sample() {
    let doc = demo();
    let camera = Viewport2d::default();
    let model = addressed(FEM2D_UTILITY_SELECT_MARQUEE);
    pointer_down(&doc, &camera, &model, 12.0, 12.0, CANVAS_WIDTH, CANVAS_HEIGHT, 0, "replace").expect("down");
    let samples = [(200.0, 300.0), (600.0, 100.0), (CANVAS_WIDTH - 12.0, CANVAS_HEIGHT - 12.0)];
    pointer_move(&doc, &camera, &model, &samples, CANVAS_WIDTH, CANVAS_HEIGHT).expect("batched move");
    let gesture = gesture_for_window("w");
    let layers = fem2d_marquee_overlay_layers(&gesture);
    clear_gesture("w");
    assert_eq!(layers.len(), 1);
    let segments = layers[0].get("segments").and_then(|value| value.as_array()).expect("segments");
    let corner = segments.get(2).and_then(|segment| segment.get("to")).and_then(|value| value.as_array()).expect("far corner");
    let expected = canvas_to_layer(&camera, CANVAS_WIDTH - 12.0, CANVAS_HEIGHT - 12.0, CANVAS_WIDTH, CANVAS_HEIGHT);
    assert!((corner[0].as_f64().unwrap() - expected.0).abs() < 1e-9 && (corner[1].as_f64().unwrap() - expected.1).abs() < 1e-9, "the rectangle spans start → LAST sample, not an intermediate one");
    let hits = fem2d_marquee_hits(&doc, &camera, &gesture.method, &gesture.canvas_points, CANVAS_WIDTH, CANVAS_HEIGHT);
    assert!(hits.iter().any(|(_, id)| id == "n1") && hits.iter().any(|(_, id)| id == "n2"));
}

#[test]
fn a_cancelled_release_selects_nothing_and_leaves_no_gesture() {
    let doc = demo();
    let camera = Viewport2d::default();
    let model = addressed(FEM2D_UTILITY_SELECT_MARQUEE);
    let end = (CANVAS_WIDTH - 12.0, CANVAS_HEIGHT - 12.0);
    pointer_down(&doc, &camera, &model, 12.0, 12.0, CANVAS_WIDTH, CANVAS_HEIGHT, 0, "replace").expect("down");
    pointer_move(&doc, &camera, &model, &[end], CANVAS_WIDTH, CANVAS_HEIGHT).expect("move");
    assert!(gesture_for_window("w").active, "the marquee is live before the cancel");
    let emit = pointer_up(&doc, &camera, &model, end.0, end.1, CANVAS_WIDTH, CANVAS_HEIGHT, "replace", true).expect("cancel");
    assert!(emit.effects.is_empty(), "a cancel never selects or picks");
    assert!(emit.artifact_mutations.is_empty() && emit.window_config_mutations.is_empty());
    assert!(matches!(emit.ui_scope, UiDirtyScope::Partial { .. }), "the overlay is refreshed away");
    assert_eq!(gesture_for_window("w"), Fem2dCanvasGesture::default(), "no gesture survives a cancel");

    // 🎯️ A cancel over a node in direct-select mode does not fall back to a pick either.
    let direct = addressed(FEM2D_UTILITY_SELECT_DIRECT);
    let n1 = find_node_2d(&doc.nodes, "n1").expect("n1");
    let (x, y) = fem2d_layer_to_canvas(&camera, screen_2d(n1.x, n1.y), CANVAS_WIDTH, CANVAS_HEIGHT);
    let emit = pointer_up(&doc, &camera, &direct, x, y, CANVAS_WIDTH, CANVAS_HEIGHT, "replace", true).expect("cancel");
    assert!(emit.effects.is_empty(), "no pick on a cancelled release");
}
//#endregion 🧪️BatchedSamples
