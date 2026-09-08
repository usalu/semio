
use super::*;
use crate::editor::fem2d::Fem2dCommand;
use crate::editor::fem2d::testkit::{dispatch, fem2d_app, render as render_body};

async fn load_default_example(app: &mut crate::editor::fem2d::testkit::Fem2dApp) {
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
    let camera = FemCamera::default();
    let json = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(render(&doc, &display, &camera).expect("fixture surface admission"))).expect("fixture projection");
    assert!(json.contains("No load case defined"), "{json}");
}

#[semio_framework_async_macros::async_test]
async fn results_window_renders_contour_for_region() {
    let mut app = fem2d_app();
    load_default_example(&mut app).await;
    let snapshot = app.snapshot().expect("snapshot");
    let node = render(&snapshot, &ResultDisplay { source_id: Some("dead".into()), mode: DisplayMode::Static }, &FemCamera::default()).expect("fixture surface admission");
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
    let node = render(&snapshot, &ResultDisplay { source_id: Some("dead".into()), mode: DisplayMode::Static }, &FemCamera::default()).expect("fixture surface admission");
    let semio_framework_ui_contract::Component::Surface(props) = &node.component else { panic!("expected canvas surface") };
    let scene: Canvas2dScene = semio_framework_ui_scene::decode(props).expect("decode canvas scene");
    assert!(scene.layers_json.contains("reaction-"), "expected reaction-prefixed text label layers: {}", scene.layers_json);
}

#[semio_framework_async_macros::async_test]
async fn results_window_renders_modal_mode_shape_2d() {
    let mut app = fem2d_app();
    load_default_example(&mut app).await;
    dispatch(&mut app, Fem2dCommand::SetResultDisplay(crate::editor::fem2d::commands::set_result_display::SetResultDisplay { source_id: None, mode: "modal".into(), mode_index: 0 })).await;
    let json = render_body(&mut app, BODY_KEY);
    assert!(json.contains("canvas-2d"), "expected a valid canvas-2d scene, got: {json}");
    assert!(!json.contains("Modal analysis error"), "unexpected modal error: {json}");
}

#[semio_framework_async_macros::async_test]
async fn results_window_renders_buckling_mode_shape_2d() {
    let mut app = fem2d_app();
    load_default_example(&mut app).await;
    dispatch(&mut app, Fem2dCommand::SetResultDisplay(crate::editor::fem2d::commands::set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "buckling".into(), mode_index: 0 })).await;
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
