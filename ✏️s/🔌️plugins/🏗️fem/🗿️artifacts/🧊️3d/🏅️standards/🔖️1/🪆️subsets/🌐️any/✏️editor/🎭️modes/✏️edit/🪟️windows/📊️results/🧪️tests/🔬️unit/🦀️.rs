use super::*;
use crate::editor::fem3d::unit_tests::context::{dispatch, fem3d_app, fem3d_empty_app, render as render_body, Fem3dApp};
use crate::editor::fem3d::Fem3dCommand;

async fn app_with_example() -> Fem3dApp {
    let mut app = fem3d_app();
    dispatch(&mut app, Fem3dCommand::SetActiveExample(crate::editor::fem3d::commands::set_active_example::SetActiveExample { example_id: "default".into() })).await;
    app
}

#[semio_framework_async_macros::async_test]
async fn renders_fem3d_results_scene() {
    let mut app = app_with_example().await;
    let json = render_body(&mut app, FEM3D_BODY_RESULTS);
    assert!(json.contains("world-3d"));
}

#[semio_framework_async_macros::async_test]
async fn results_window_surfaces_solver_error_without_panicking_3d() {
    let mut app = fem3d_empty_app().await;
    let _ = render_body(&mut app, FEM3D_BODY_RESULTS);
}

#[semio_framework_async_macros::async_test]
async fn results_window_renders_modal_mode_shape_3d() {
    let mut app = app_with_example().await;
    dispatch(&mut app, Fem3dCommand::SetResultDisplay(crate::editor::fem3d::commands::set_result_display::SetResultDisplay { source_id: None, mode: "modal".into(), mode_index: 0 })).await;
    let json = render_body(&mut app, FEM3D_BODY_RESULTS);
    assert!(json.contains("world-3d"), "expected a valid world-3d scene, got: {json}");
    assert!(!json.contains("Modal analysis error"), "unexpected modal error: {json}");
}

#[semio_framework_async_macros::async_test]
async fn results_window_renders_buckling_mode_shape_3d() {
    let mut app = app_with_example().await;
    dispatch(&mut app, Fem3dCommand::SetResultDisplay(crate::editor::fem3d::commands::set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "buckling".into(), mode_index: 0 })).await;
    let json = render_body(&mut app, FEM3D_BODY_RESULTS);
    assert!(json.contains("world-3d"), "expected a valid world-3d scene, got: {json}");
    assert!(!json.contains("Buckling analysis error"), "unexpected buckling error: {json}");
}

#[semio_framework_async_macros::async_test]
async fn results_scene_includes_solid_vertex_colors_3d() {
    let app = app_with_example().await;
    let snapshot = app.snapshot().expect("snapshot");
    let config = Fem3dResultsWindowConfig { result_source_id: Some("dead".into()), result_mode: crate::app_surface::ResultMode::Static, ..Fem3dResultsWindowConfig::default() };
    let node = render(&snapshot, &config).expect("fixture surface admission");
    let surface = node.children.iter().find(|child| matches!(&child.component, semio_framework_ui_contract::Component::Surface(_))).expect("world surface child");
    let scene: semio_framework_ui_scene::World3dScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(surface).expect("assemble world scene");
    let json = serde_json::to_string(&node).expect("independent semantic JSON oracle");
    assert!(scene.meshes_json.contains("solid-sol1"), "expected the solid mesh in the results scene: {}", scene.meshes_json);
    assert!(scene.meshes_json.contains("\"colors\""), "expected a vertex colors array on the solid mesh data: {}", scene.meshes_json);
    assert!(json.contains("Case: dead"), "expected a case-id caption: {json}");
}

#[semio_framework_async_macros::async_test]
async fn results_scene_captions_name_mode_and_factor_3d() {
    let mut app = app_with_example().await;
    dispatch(&mut app, Fem3dCommand::SetResultDisplay(crate::editor::fem3d::commands::set_result_display::SetResultDisplay { source_id: None, mode: "modal".into(), mode_index: 0 })).await;
    let json_modal = render_body(&mut app, FEM3D_BODY_RESULTS);
    assert!(json_modal.contains("Hz"), "expected a frequency caption: {json_modal}");

    dispatch(&mut app, Fem3dCommand::SetResultDisplay(crate::editor::fem3d::commands::set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "buckling".into(), mode_index: 0 })).await;
    let json_buckling = render_body(&mut app, FEM3D_BODY_RESULTS);
    assert!(json_buckling.contains("factor"), "expected a load-factor caption: {json_buckling}");
}

#[semio_framework_async_macros::async_test]
async fn fem3d_model_extent_degenerate_model_returns_one() {
    assert_eq!(fem3d_model_extent(&Fem3dSnapshot::default()), 1.0);
}
