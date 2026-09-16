use super::*;
use crate::editor::fem2d::interaction::canvas_gesture::FEM2D_UTILITY_SELECT_DIRECT;

/// 📊️ One results scene rendered straight off a document and a window configuration.
///
/// No live app: a `VcsArtifactApp` that has opened a document owes the store a terminal close, and
/// the framework's own fixture close walks a one-item budget the demo-sized store does not fit. A
/// render law needs neither — `render` is a pure function of the snapshot, the display, the window
/// configuration and the interaction snapshot.
fn results_scene(doc: &Fem2dSnapshot, display: &ResultDisplay, window: &config::Fem2dResultsWindowConfig) -> Canvas2dScene {
    let node = render(doc, display, &Viewport2d::default(), window, &crate::editor::fem2d::interaction::Fem2dInteractionSnapshot::default(), None, None, FEM2D_UTILITY_SELECT_DIRECT).expect("fixture surface admission");
    let semio_framework_ui_contract::Component::Surface(props) = &node.component else { panic!("expected canvas surface") };
    semio_framework_ui_scene::decode(props).expect("decode canvas scene")
}

fn demo() -> Fem2dSnapshot {
    crate::standards::v1::subsets::any::schema::default_fem2d_snapshot()
}

#[semio_framework_async_macros::async_test]
async fn renders_fem2d_results_scene() {
    let node = render(&demo(), &ResultDisplay { source_id: None, mode: DisplayMode::Static }, &Viewport2d::default(), &config::Fem2dResultsWindowConfig::default(), &crate::editor::fem2d::interaction::Fem2dInteractionSnapshot::default(), None, None, FEM2D_UTILITY_SELECT_DIRECT).expect("fixture surface admission");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("fixture projection");
    assert!(json.contains("canvas-2d"), "{json}");
}

#[semio_framework_async_macros::async_test]
async fn results_window_surfaces_solver_error_without_panicking_2d() {
    let empty = crate::standards::v1::subsets::any::schema::empty_fem2d_snapshot();
    let node = render(&empty, &ResultDisplay { source_id: None, mode: DisplayMode::Static }, &Viewport2d::default(), &config::Fem2dResultsWindowConfig::default(), &crate::editor::fem2d::interaction::Fem2dInteractionSnapshot::default(), None, None, FEM2D_UTILITY_SELECT_DIRECT).expect("a refused analysis still admits a surface");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("fixture projection");
    assert!(!json.is_empty(), "a document the solver refuses surfaces its message instead of panicking");
}

#[semio_framework_async_macros::async_test]
async fn results_window_buckling_with_no_load_case_shows_placeholder_2d() {
    let doc = crate::standards::v1::subsets::any::schema::empty_fem2d_snapshot();
    let display = ResultDisplay { source_id: None, mode: DisplayMode::Buckling(0) };
    let camera = Viewport2d::default();
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(render(&doc, &display, &camera, &config::Fem2dResultsWindowConfig::default(), &crate::editor::fem2d::interaction::Fem2dInteractionSnapshot::default(), None, None, FEM2D_UTILITY_SELECT_DIRECT).expect("fixture surface admission"))).expect("fixture projection");
    assert!(json.contains("No load case defined"), "{json}");
}

#[semio_framework_async_macros::async_test]
async fn results_window_renders_contour_for_region() {
    let scene = results_scene(&demo(), &ResultDisplay { source_id: Some("dead".into()), mode: DisplayMode::Static }, &config::Fem2dResultsWindowConfig::default());
    assert!(scene.layers_json.contains("fill"), "expected filled-path contour layers for the region's Tri3Cst elements: {}", scene.layers_json);
    assert!(scene.layers_json.contains("contour-"), "expected contour-prefixed layer ids: {}", scene.layers_json);
}

#[semio_framework_async_macros::async_test]
async fn results_window_renders_reaction_labels_2d() {
    let scene = results_scene(&demo(), &ResultDisplay { source_id: Some("dead".into()), mode: DisplayMode::Static }, &config::Fem2dResultsWindowConfig::default());
    assert!(scene.layers_json.contains("reaction-"), "expected reaction-prefixed text label layers: {}", scene.layers_json);
}

/// 📍️ LAW: multiple DOFs at one support node stack vertically instead of sharing one transform.
#[semio_framework_async_macros::async_test]
async fn reaction_labels_at_one_node_use_distinct_positions() {
    let scene = results_scene(&demo(), &ResultDisplay { source_id: Some("dead".into()), mode: DisplayMode::Static }, &config::Fem2dResultsWindowConfig::default());
    let layers: Vec<serde_json::Value> = serde_json::from_str(&scene.layers_json).expect("layers json");
    let mut by_node: std::collections::HashMap<String, Vec<f64>> = std::collections::HashMap::new();
    for layer in layers {
        let id = layer.get("id").and_then(|value| value.as_str()).unwrap_or("");
        if !id.starts_with("reaction-") {
            continue;
        }
        let transform = layer.get("transform").and_then(|value| value.as_array()).expect("text transform");
        let y = transform[5].as_f64().expect("transform ty");
        let node_id = id.strip_prefix("reaction-").and_then(|rest| rest.rsplit_once('-')).map(|(node, _)| node).expect("reaction id");
        by_node.entry(node_id.to_string()).or_default().push(y);
    }
    for (node, ys) in by_node {
        if ys.len() < 2 {
            continue;
        }
        let distinct = ys.iter().fold(std::collections::HashSet::new(), |mut set, y| {
            set.insert(y.to_bits());
            set
        })
        .len();
        assert_eq!(distinct, ys.len(), "node {node} reaction labels must not overlap vertically: {ys:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn place_reaction_label_stacks_then_avoids_overlap() {
    let mut placed = Vec::new();
    let (x0, y0) = place_reaction_label(40.0, 50.0, 0, "Tx: 100 N", &mut placed);
    let (x1, y1) = place_reaction_label(40.0, 50.0, 1, "Ty: 200 N", &mut placed);
    assert_eq!(x0, x1);
    assert!(y1 > y0, "second dof at the same node sits below the first");
    let (x2, y2) = place_reaction_label(42.0, 51.0, 0, "Tx: 300 N", &mut placed);
    assert!(y2 > y0 || x2 != x0 || (y2 - y0).abs() > 1e-6, "nearby nodes nudge apart when stacked positions would collide");
}

#[semio_framework_async_macros::async_test]
async fn results_window_renders_modal_mode_shape_2d() {
    let scene = results_scene(&demo(), &ResultDisplay { source_id: None, mode: DisplayMode::Modal(0) }, &config::Fem2dResultsWindowConfig::default());
    assert!(scene.layers_json.contains("modal-caption"), "expected the mode's frequency caption: {}", scene.layers_json);
    assert!(!scene.layers_json.contains("Modal analysis error"), "unexpected modal error: {}", scene.layers_json);
}

#[semio_framework_async_macros::async_test]
async fn results_window_renders_buckling_mode_shape_2d() {
    let scene = results_scene(&demo(), &ResultDisplay { source_id: Some("dead".into()), mode: DisplayMode::Buckling(0) }, &config::Fem2dResultsWindowConfig::default());
    assert!(scene.layers_json.contains("buckling-caption"), "expected the mode's load-factor caption: {}", scene.layers_json);
    assert!(!scene.layers_json.contains("Buckling analysis error"), "unexpected buckling error: {}", scene.layers_json);
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
    let node = render(&doc, &display, &Viewport2d::default(), &window, &crate::editor::fem2d::interaction::Fem2dInteractionSnapshot::default(), None, None, FEM2D_UTILITY_SELECT_DIRECT).expect("fixture surface admission");
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

/// 🎵️ LAW: the mode-shape views read the SAME waveform amplitude the static view does — half the
/// phase is half the amplitude, and a sine trough swings the mode the other way.
///
/// Asserted on the scale the render feeds `fem2d_deformed_shape_layers`, not on the drawn bytes: the
/// demo's first eigenmode is a slab mode whose frame nodes barely move, so its polylines are
/// byte-identical at every amplitude and a pixel comparison would prove nothing either way.
#[semio_framework_async_macros::async_test]
async fn results_window_mode_shapes_follow_the_playback_phase() {
    let doc = demo();
    let full = mode_shape_scale(&doc, Fem2dResultsAnimation::default().amplitude());
    assert!(full > 0.0, "the demo has a non-degenerate extent");
    assert!((mode_shape_scale(&doc, Fem2dResultsAnimation { phase: 0.5, ..Fem2dResultsAnimation::default() }.amplitude()) - full / 2.0).abs() < 1e-9);
    let trough = Fem2dResultsAnimation { phase: 0.75, waveform: Fem2dWaveform::Sine, ..Fem2dResultsAnimation::default() };
    assert!(mode_shape_scale(&doc, trough.amplitude()) < 0.0, "a sine trough swings the mode the other way");
}

/// ⏯️ LAW: a running mode-shape window carries the same transport read-out the static view does.
#[semio_framework_async_macros::async_test]
async fn results_window_mode_shapes_carry_the_transport_caption_while_playing() {
    let running = animated_scene(Fem2dResultsAnimation { phase: 0.42, playing: true, ..Fem2dResultsAnimation::default() }, DisplayMode::Modal(0));
    assert!(running.layers_json.contains("playback-caption"), "{}", running.layers_json);
    assert!(running.layers_json.contains("modal-caption"), "{}", running.layers_json);
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
