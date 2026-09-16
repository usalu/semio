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
    let json = render_body(&mut app, FEM3D_BODY_RESULTS);
    assert!(json.contains("No load case defined"), "an empty document names its missing case: {json}");
    assert!(json.contains("world-3d"), "the structure stays visible under the fault caption: {json}");
}

/// 🧠️ LAW: one revision is solved ONCE however many frames playback draws over it, a new revision
/// drops the old entry, and a render outside any operation never caches. Asserted through the
/// cache API with an explicit key, the way the render reads it.
#[semio_framework_async_macros::async_test]
async fn results_cache_solves_one_revision_once_and_evicts_the_previous() {
    let doc = <Fem3dSnapshot as store::ArtifactDsl>::parse_dsl(crate::standards::v1::subsets::any::schema::snapshot::text::FEM3D_EXAMPLE_TEXT).expect("default example snapshot");
    reset_results_cache();
    let key = Some((7_u32, [1_u8; 32]));
    for _ in 0..30 {
        let cases = with_static_results(&doc, key, |(results, stresses)| (results.len(), stresses.len())).expect("static results");
        assert!(cases.0 > 0 && cases.1 == cases.0, "every solved case carries its stress field");
    }
    assert_eq!(results_solve_count(), 1, "thirty playback frames over one revision cost one solve");
    with_mode_values(&doc, key, ModeKey::Modal(0), |(frequency, _)| assert!(*frequency > 0.0)).expect("modal values");
    with_mode_values(&doc, key, ModeKey::Modal(0), |(frequency, _)| assert!(*frequency > 0.0)).expect("modal values");
    assert_eq!(results_solve_count(), 2, "the mode shape is solved once too");
    let moved = Some((7_u32, [2_u8; 32]));
    with_static_results(&doc, moved, |(results, _)| results.len()).expect("static results");
    assert_eq!(results_solve_count(), 3, "a moved revision re-solves");
    RESULTS_CACHE.with(|cache| {
        let cache = cache.borrow();
        let entry = cache.as_ref().expect("one resident entry");
        assert_eq!(entry.key, moved.expect("key"));
        assert!(entry.modes.is_empty(), "the previous revision's mode shapes are dropped, never kept alongside");
    });
    reset_results_cache();
    with_static_results(&doc, None, |(results, _)| results.len()).expect("static results");
    with_static_results(&doc, None, |(results, _)| results.len()).expect("static results");
    assert_eq!(results_solve_count(), 2, "a render outside any operation never caches");
    reset_results_cache();
}

/// 🎞️ LAW: the playback phase scales the drawn deformation — phase 1 and phase 0 draw different
/// poses of the same solved revision, and a running window carries its transport read-out.
#[semio_framework_async_macros::async_test]
async fn playback_phase_moves_the_deformed_scene() {
    let snapshot = <Fem3dSnapshot as store::ArtifactDsl>::parse_dsl(crate::standards::v1::subsets::any::schema::snapshot::text::FEM3D_EXAMPLE_TEXT).expect("default example snapshot");
    let interaction = crate::editor::fem3d::interaction::Fem3dInteractionSnapshot::default();
    let instances = |node: &semio_framework_plugin::BuiltNode| {
        let surface = node.children[1].children.iter().find(|child| matches!(&child.component, semio_framework_ui_contract::Component::Surface(_))).expect("world surface child");
        semio_framework_plugin::artifact_app_laws::built_surface_scene(surface).expect("scene").instances_json
    };
    let mut config = Fem3dResultsWindowConfig::default();
    let full = render(&snapshot, &config, &interaction, None).expect("full frame");
    config.animation.phase = 0.0;
    let rest = render(&snapshot, &config, &interaction, None).expect("rest frame");
    assert_ne!(instances(&full), instances(&rest), "phase 1 and phase 0 draw different poses");
    config.animation.playing = true;
    config.animation.phase = 0.42;
    let running = render(&snapshot, &config, &interaction, None).expect("running frame");
    let json = serde_json::to_string(&running).expect("json");
    assert!(json.contains("phase 0.42"), "a running window carries its transport read-out: {json}");
}

#[semio_framework_async_macros::async_test]
async fn results_window_renders_modal_mode_shape_3d() {
    let mut app = app_with_example().await;
    dispatch(&mut app, Fem3dCommand::SetResultDisplay(crate::editor::fem3d::commands::set_result_display::SetResultDisplay { source_id: None, mode: "modal".into(), mode_index: 0, field: None, value: None, window_id: None })).await;
    let json = render_body(&mut app, FEM3D_BODY_RESULTS);
    assert!(json.contains("world-3d"), "expected a valid world-3d scene, got: {json}");
    assert!(!json.contains("Modal analysis error"), "unexpected modal error: {json}");
}

#[semio_framework_async_macros::async_test]
async fn results_window_renders_buckling_mode_shape_3d() {
    let mut app = app_with_example().await;
    dispatch(&mut app, Fem3dCommand::SetResultDisplay(crate::editor::fem3d::commands::set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "buckling".into(), mode_index: 0, field: None, value: None, window_id: None })).await;
    let json = render_body(&mut app, FEM3D_BODY_RESULTS);
    assert!(json.contains("world-3d"), "expected a valid world-3d scene, got: {json}");
    assert!(!json.contains("Buckling analysis error"), "unexpected buckling error: {json}");
}

#[semio_framework_async_macros::async_test]
async fn results_caption_column_grows_so_world_scene_fills_window_3d() {
    let snapshot = <Fem3dSnapshot as store::ArtifactDsl>::parse_dsl(crate::standards::v1::subsets::any::schema::snapshot::text::FEM3D_EXAMPLE_TEXT).expect("default example snapshot");
    let config = Fem3dResultsWindowConfig::default();
    let node = render(&snapshot, &config, &crate::editor::fem3d::interaction::Fem3dInteractionSnapshot::default(), None).expect("fixture surface admission");
    let stack_grows = |layout: &semio_framework_ui_contract::LayoutSpec| {
        matches!(layout, semio_framework_ui_contract::LayoutSpec::Stack(stack) if stack.grow)
    };
    assert!(stack_grows(&node.layout), "outer caption column must grow to fill the window body");
    let scene_stack = node.children.get(1).expect("caption + growing scene stack");
    assert!(stack_grows(&scene_stack.layout), "scene stack must grow so the world-3d host fills space below the caption");
}

#[semio_framework_async_macros::async_test]
async fn results_scene_includes_solid_vertex_colors_3d() {
    let app = app_with_example().await;
    let snapshot = app.snapshot().expect("snapshot");
    let config = Fem3dResultsWindowConfig { result_source_id: Some("dead".into()), result_mode: crate::app_surface::ResultMode::Static, ..Fem3dResultsWindowConfig::default() };
    let node = render(&snapshot, &config, &crate::editor::fem3d::interaction::Fem3dInteractionSnapshot::default(), None).expect("fixture surface admission");
    let scene_stack = node.children.get(1).expect("caption + growing scene stack");
    let surface = scene_stack.children.iter().find(|child| matches!(&child.component, semio_framework_ui_contract::Component::Surface(_))).expect("world surface child");
    let scene: semio_framework_ui_scene::World3dScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(surface).expect("assemble world scene");
    let json = serde_json::to_string(&node).expect("independent semantic JSON oracle");
    assert!(scene.meshes_json.contains("solid-sol1"), "expected the solid mesh in the results scene: {}", scene.meshes_json);
    assert!(scene.meshes_json.contains("\"colors\""), "expected a vertex colors array on the solid mesh data: {}", scene.meshes_json);
    assert!(json.contains("Case: dead"), "expected a case-id caption: {json}");
}

#[semio_framework_async_macros::async_test]
async fn results_scene_captions_name_mode_and_factor_3d() {
    let mut app = app_with_example().await;
    dispatch(&mut app, Fem3dCommand::SetResultDisplay(crate::editor::fem3d::commands::set_result_display::SetResultDisplay { source_id: None, mode: "modal".into(), mode_index: 0, field: None, value: None, window_id: None })).await;
    let json_modal = render_body(&mut app, FEM3D_BODY_RESULTS);
    assert!(json_modal.contains("Hz"), "expected a frequency caption: {json_modal}");

    dispatch(&mut app, Fem3dCommand::SetResultDisplay(crate::editor::fem3d::commands::set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "buckling".into(), mode_index: 0, field: None, value: None, window_id: None })).await;
    let json_buckling = render_body(&mut app, FEM3D_BODY_RESULTS);
    assert!(json_buckling.contains("factor"), "expected a load-factor caption: {json_buckling}");
}

#[semio_framework_async_macros::async_test]
async fn fem3d_model_extent_degenerate_model_returns_one() {
    assert_eq!(fem3d_model_extent(&Fem3dSnapshot::default()), 1.0);
}
