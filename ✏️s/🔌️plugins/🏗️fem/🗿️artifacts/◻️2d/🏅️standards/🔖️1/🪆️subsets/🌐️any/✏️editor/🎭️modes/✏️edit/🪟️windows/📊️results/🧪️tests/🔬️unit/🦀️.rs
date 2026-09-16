use super::*;
use crate::editor::fem2d::unit_tests::context::{dispatch, fem2d_app, render as render_body};
use crate::editor::fem2d::Fem2dCommand;

async fn load_default_example(app: &mut crate::editor::fem2d::unit_tests::context::Fem2dApp) {
    dispatch(app, Fem2dCommand::SetActiveExample(crate::editor::fem2d::commands::set_active_example::SetActiveExample { example_id: "default".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn renders_fem2d_results_scene() {
    let mut app = fem2d_app();
    load_default_example(&mut app).await;
    assert!(render_body(&mut app, BODY_KEY).contains("canvas-2d"));
}

#[semio_framework_async_macros::async_test]
async fn results_window_surfaces_solver_error_without_panicking_2d() {
    let mut app = fem2d_app();
    let _ = render_body(&mut app, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn results_window_buckling_with_no_load_case_shows_placeholder_2d() {
    let doc = crate::standards::v1::subsets::any::schema::empty_fem2d_snapshot();
    let display = ResultDisplay { source_id: None, mode: DisplayMode::Buckling(0) };
    let camera = Viewport2d::default();
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(render(&doc, &display, &camera, &config::Fem2dResultsWindowConfig::default(), &crate::editor::fem2d::interaction::Fem2dInteractionSnapshot::default(), None).expect("fixture surface admission"))).expect("fixture projection");
    assert!(json.contains("No load case defined"), "{json}");
}

#[semio_framework_async_macros::async_test]
async fn results_window_renders_contour_for_region() {
    let mut app = fem2d_app();
    load_default_example(&mut app).await;
    let snapshot = app.snapshot().expect("snapshot");
    let node = render(&snapshot, &ResultDisplay { source_id: Some("dead".into()), mode: DisplayMode::Static }, &Viewport2d::default(), &config::Fem2dResultsWindowConfig::default(), &crate::editor::fem2d::interaction::Fem2dInteractionSnapshot::default(), None).expect("fixture surface admission");
    let semio_framework_ui_contract::Component::Surface(props) = &node.component else { panic!("expected canvas surface") };
    let scene: Canvas2dScene = semio_framework_ui_scene::decode(props).expect("decode canvas scene");
    assert!(scene.layers_json.contains("fill"), "expected filled-path contour layers for the region's Tri3Cst elements: {}", scene.layers_json);
    assert!(scene.layers_json.contains("contour-"), "expected contour-prefixed layer ids: {}", scene.layers_json);
}

#[semio_framework_async_macros::async_test]
async fn results_window_renders_reaction_labels_2d() {
    let mut app = fem2d_app();
    load_default_example(&mut app).await;
    let snapshot = app.snapshot().expect("snapshot");
    let node = render(&snapshot, &ResultDisplay { source_id: Some("dead".into()), mode: DisplayMode::Static }, &Viewport2d::default(), &config::Fem2dResultsWindowConfig::default(), &crate::editor::fem2d::interaction::Fem2dInteractionSnapshot::default(), None).expect("fixture surface admission");
    let semio_framework_ui_contract::Component::Surface(props) = &node.component else { panic!("expected canvas surface") };
    let scene: Canvas2dScene = semio_framework_ui_scene::decode(props).expect("decode canvas scene");
    assert!(scene.layers_json.contains("reaction-"), "expected reaction-prefixed text label layers: {}", scene.layers_json);
}

#[semio_framework_async_macros::async_test]
async fn results_window_renders_modal_mode_shape_2d() {
    let mut app = fem2d_app();
    load_default_example(&mut app).await;
    dispatch(&mut app, Fem2dCommand::SetResultDisplay(crate::editor::fem2d::commands::set_result_display::SetResultDisplay { source_id: None, mode: "modal".into(), mode_index: 0, field: None, value: None })).await;
    let json = render_body(&mut app, BODY_KEY);
    assert!(json.contains("canvas-2d"), "expected a valid canvas-2d scene, got: {json}");
    assert!(!json.contains("Modal analysis error"), "unexpected modal error: {json}");
}

#[semio_framework_async_macros::async_test]
async fn results_window_renders_buckling_mode_shape_2d() {
    let mut app = fem2d_app();
    load_default_example(&mut app).await;
    dispatch(&mut app, Fem2dCommand::SetResultDisplay(crate::editor::fem2d::commands::set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "buckling".into(), mode_index: 0, field: None, value: None })).await;
    let json = render_body(&mut app, BODY_KEY);
    assert!(json.contains("canvas-2d"), "expected a valid canvas-2d scene, got: {json}");
    assert!(!json.contains("Buckling analysis error"), "unexpected buckling error: {json}");
}

#[semio_framework_async_macros::async_test]
async fn interpolate_at_value_falls_back_to_midpoint_when_values_equal() {
    let (point, value) = interpolate_at_value(((0.0, 0.0), 5.0), ((10.0, 20.0), 5.0), 5.0);
    assert_eq!(point, (5.0, 10.0));
    assert_eq!(value, 5.0);
}

#[semio_framework_async_macros::async_test]
async fn clip_by_value_empty_polygon_returns_empty() {
    assert!(clip_by_value(&[], 0.0, true).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn clip_by_value_keeps_only_the_requested_half_plane() {
    let poly: Vec<ValuedPoint> = vec![((0.0, 0.0), 0.0), ((10.0, 0.0), 10.0), ((0.0, 10.0), 0.0)];
    let above = clip_by_value(&poly, 5.0, true);
    assert!(above.len() >= 3 && above.iter().all(|(_, v)| *v >= 5.0 - 1e-9));
    let below = clip_by_value(&poly, 5.0, false);
    assert!(below.len() >= 3 && below.iter().all(|(_, v)| *v <= 5.0 + 1e-9));
}

//#region 🔖️Animation
use crate::editor::fem2d::modes::edit::windows::results::config::{Fem2dResultsAnimation, Fem2dWaveform};

fn animated_scene(animation: Fem2dResultsAnimation, mode: DisplayMode) -> Canvas2dScene {
    let doc = crate::standards::v1::subsets::any::schema::default_fem2d_snapshot();
    let window = config::Fem2dResultsWindowConfig { animation, ..config::Fem2dResultsWindowConfig::default() };
    let display = ResultDisplay { source_id: Some("dead".into()), mode };
    let node = render(&doc, &display, &Viewport2d::default(), &window, &crate::editor::fem2d::interaction::Fem2dInteractionSnapshot::default(), None).expect("fixture surface admission");
    let semio_framework_ui_contract::Component::Surface(props) = &node.component else { panic!("expected canvas surface") };
    semio_framework_ui_scene::decode(props).expect("decode canvas scene")
}

/// 〰️ LAW: the phase actually moves the structure — a sine frame at quarter phase is a different
/// scene from the zero-crossing at phase 0, and the reaction read-outs ride the same factor.
#[semio_framework_async_macros::async_test]
async fn results_window_static_scene_follows_the_playback_phase() {
    let sine = Fem2dResultsAnimation { waveform: Fem2dWaveform::Sine, ..Fem2dResultsAnimation::default() };
    let peak = animated_scene(Fem2dResultsAnimation { phase: 0.25, ..sine }, DisplayMode::Static);
    let rest = animated_scene(Fem2dResultsAnimation { phase: 0.0, ..sine }, DisplayMode::Static);
    assert_ne!(peak.layers_json, rest.layers_json, "phase 0.25 and phase 0 must not draw the same frame");
    let trough = animated_scene(Fem2dResultsAnimation { phase: 0.75, ..sine }, DisplayMode::Static);
    assert_ne!(peak.layers_json, trough.layers_json, "a sine swings through both signs");
}

/// 🎞️ LAW: the default (still) window draws the FULL deformation — the pre-playback behaviour.
#[semio_framework_async_macros::async_test]
async fn results_window_default_animation_draws_the_full_deformation() {
    let still = animated_scene(Fem2dResultsAnimation::default(), DisplayMode::Static);
    let full = animated_scene(Fem2dResultsAnimation { phase: 1.0, ..Fem2dResultsAnimation::default() }, DisplayMode::Static);
    assert_eq!(still.layers_json, full.layers_json);
    assert!(!still.layers_json.contains("playback-caption"), "a still window shows no transport read-out");
}

/// ⏯️ LAW: a running window carries its own phase/speed read-out.
#[semio_framework_async_macros::async_test]
async fn results_window_running_scene_carries_a_transport_caption() {
    let running = animated_scene(Fem2dResultsAnimation { phase: 0.42, playing: true, ..Fem2dResultsAnimation::default() }, DisplayMode::Static);
    assert!(running.layers_json.contains("playback-caption"), "{}", running.layers_json);
    assert!(running.layers_json.contains("phase 0.42"), "{}", running.layers_json);
}

/// 🎵️ LAW: the mode-shape views animate their amplitude the same way the static view does.
#[semio_framework_async_macros::async_test]
async fn results_window_mode_shapes_follow_the_playback_phase() {
    let half = animated_scene(Fem2dResultsAnimation { phase: 0.5, ..Fem2dResultsAnimation::default() }, DisplayMode::Modal(0));
    let full = animated_scene(Fem2dResultsAnimation::default(), DisplayMode::Modal(0));
    assert_ne!(half.layers_json, full.layers_json, "the modal amplitude must ride the phase");
}

/// 🧠️ LAW: one revision is solved ONCE however many frames playback draws over it, and a new
/// revision drops the old entry instead of growing a second one.
#[semio_framework_async_macros::async_test]
async fn results_cache_solves_one_revision_once_and_evicts_the_previous() {
    let doc = crate::standards::v1::subsets::any::schema::default_fem2d_snapshot();
    reset_results_cache();
    let key = Some((7_u32, [1_u8; 32]));
    for _ in 0..30 {
        let cases = with_static_results(&doc, key, |results| results.len()).expect("static results");
        assert!(cases > 0);
    }
    assert_eq!(results_solve_count(), 1, "thirty playback frames over one revision cost one solve");
    with_mode_values(&doc, key, ModeKey::Modal(0), |(frequency, _)| assert!(*frequency > 0.0)).expect("modal values");
    with_mode_values(&doc, key, ModeKey::Modal(0), |(frequency, _)| assert!(*frequency > 0.0)).expect("modal values");
    assert_eq!(results_solve_count(), 2, "the mode shape is solved once too");
    let moved = Some((7_u32, [2_u8; 32]));
    with_static_results(&doc, moved, |results| results.len()).expect("static results");
    assert_eq!(results_solve_count(), 3, "a moved revision re-solves");
    RESULTS_CACHE.with(|cache| {
        let cache = cache.borrow();
        let entry = cache.as_ref().expect("one resident entry");
        assert_eq!(entry.key, moved.expect("key"));
        assert!(entry.modes.is_empty(), "the previous revision's mode shapes are dropped, never kept alongside");
    });
    reset_results_cache();
    with_static_results(&doc, None, |results| results.len()).expect("static results");
    with_static_results(&doc, None, |results| results.len()).expect("static results");
    assert_eq!(results_solve_count(), 2, "a render outside any operation never caches");
    reset_results_cache();
}
//#endregion 🔖️Animation
