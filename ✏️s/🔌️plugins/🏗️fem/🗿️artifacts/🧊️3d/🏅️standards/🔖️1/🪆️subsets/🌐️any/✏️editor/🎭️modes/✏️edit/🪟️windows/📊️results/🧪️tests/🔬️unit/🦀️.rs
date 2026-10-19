use semio_s_artifact_fem_3d_editor_results_window::editor::fem3d::modes::edit::windows::results::*;
use semio_s_artifact_fem_3d_editor_results_window::editor::fem3d::modes::edit::windows::results::config::Fem3dResultsWindowConfig;
use semio_s_fem_app_surface::normalize_mode_shape;
use semio_s_fem_app_surface::DisplayMode;
use semio_s_fem_app_surface::ResultDisplay;
use semio_s_fem_app_surface::MODE_SHAPE_AMPLITUDE_RATIO;
use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::gumball::fem3d_selection_json;
use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::Fem3dInteractionSnapshot;
use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::FEM3D_GRANULARITY_NODE;
use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::FEM3D_INTERACTION_DOMAIN;
use semio_s_artifact_fem_3d_scene::standards::v1::subsets::any::scene::fem3d_scene_parts;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_framework_plugin::BuiltNode;
use semio_framework_ui_contract::Buildable;
use semio_framework_ui_contract::HasChildren;
use semio_framework_ui_contract::HasStackLayout;
use std::collections::HashMap;
use crate::context::dispatch;
use crate::context::fem3d_app;
use crate::context::fem3d_empty_app;
use crate::context::render as render_body;
use crate::context::Fem3dApp;
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;

#[test]
fn mounted_results_frames_never_invoke_synchronous_solvers() {
    let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32768, maximum_capacity_bytes: 262144, maximum_release_bytes: 1048576, maximum_depth: 4096 };
    let operation = semio_framework_plugin::AppRenderOperationContext { app_instance_id: 9001, base_revision: semio_framework_job::RevisionId(11), generation: semio_framework_job::Generation(19), canonical_base_revision: [23; 32], mounted_policy: semio_framework_plugin::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } };
    reset_results_cache();
    let mut doc = Fem3dSnapshot::default();
    doc.analysis.modal_count = 1;
    for result_mode in [semio_s_fem_app_surface::ResultMode::Static, semio_s_fem_app_surface::ResultMode::Modal] {
        let cfg = Fem3dResultsWindowConfig { result_mode, ..Default::default() };
        for _ in 0..3 { render(&doc, &cfg, &Fem3dInteractionSnapshot::default(), Some(operation), Some("fixture-results")).expect("mounted frame"); }
    }
    assert_eq!(results_solve_count(), 0);
    assert!(matches!(crate::live_visual::close_step(operation.app_instance_id, grant), semio_framework_plugin::PluginLifecycleStep::Progress(_)));
    for _ in 0..64 { if crate::live_visual::terminal_is_empty(operation.app_instance_id) { break; } crate::live_visual::close_step(operation.app_instance_id, grant); }
    assert!(crate::live_visual::terminal_is_empty(operation.app_instance_id));
    eprintln!("[DEBUG] FEM3D mounted static/modal result frames synchronously solved zero times");
}

async fn app_with_example() -> Fem3dApp {
    let mut app = fem3d_app();
    dispatch(&mut app, Fem3dCommand::SetActiveExample(semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::set_active_example::SetActiveExample { example_id: semio_s_artifact_fem_3d_io::examples::demo::ID.into() })).await;
    app
}

#[semio_framework_async_macros::async_test]
async fn renders_fem3d_results_scene() {
    let mut app = app_with_example().await;
    let json = render_body(&mut app, FEM3D_BODY_RESULTS);
    assert!(json.contains("world-3d"));
    assert!(json.contains("\"key\":\"#0\"") && json.contains("\"key\":\"#1\""), "caption and scene stack are keyed positionally: {json}");
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
    let doc = <Fem3dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::FEM3D_EXAMPLE_TEXT).expect("default example snapshot");
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
    let snapshot = <Fem3dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::FEM3D_EXAMPLE_TEXT).expect("default example snapshot");
    let interaction = Fem3dInteractionSnapshot::default();
    let instances = |node: &BuiltNode| {
        let surface = node.children[1].children.iter().find(|child| matches!(&child.component, semio_framework_ui_contract::Component::Surface(_))).expect("world surface child");
        let scene: semio_framework_ui_scene::World3dScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(surface).expect("scene");
        scene.instances_json
    };
    let mut config = Fem3dResultsWindowConfig::default();
    let full = render(&snapshot, &config, &interaction, None, None).expect("full frame");
    config.animation.phase = 0.0;
    let rest = render(&snapshot, &config, &interaction, None, None).expect("rest frame");
    assert_ne!(instances(&full), instances(&rest), "phase 1 and phase 0 draw different poses");
    config.animation.playing = true;
    config.animation.phase = 0.42;
    let node = render(&snapshot, &config, &interaction, None, None).expect("running frame");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("fixture projection");
    assert!(json.contains("phase 0.42"), "a running window carries its transport read-out: {json}");
}

#[semio_framework_async_macros::async_test]
async fn results_window_renders_modal_mode_shape_3d() {
    let mut app = app_with_example().await;
    dispatch(&mut app, Fem3dCommand::SetResultDisplay(semio_s_artifact_fem_3d_editor::editor::fem3d::commands::set_result_display::SetResultDisplay { source_id: None, mode: "modal".into(), mode_index: 0, field: None, value: None, window_id: None })).await;
    let json = render_body(&mut app, FEM3D_BODY_RESULTS);
    assert!(json.contains("world-3d"), "expected a valid world-3d scene, got: {json}");
    assert!(!json.contains("Modal analysis error"), "unexpected modal error: {json}");
}

#[semio_framework_async_macros::async_test]
async fn results_window_renders_buckling_mode_shape_3d() {
    let mut app = app_with_example().await;
    dispatch(&mut app, Fem3dCommand::SetResultDisplay(semio_s_artifact_fem_3d_editor::editor::fem3d::commands::set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "buckling".into(), mode_index: 0, field: None, value: None, window_id: None })).await;
    let json = render_body(&mut app, FEM3D_BODY_RESULTS);
    assert!(json.contains("world-3d"), "expected a valid world-3d scene, got: {json}");
    assert!(!json.contains("Buckling analysis error"), "unexpected buckling error: {json}");
}

#[semio_framework_async_macros::async_test]
async fn results_caption_column_grows_so_world_scene_fills_window_3d() {
    let snapshot = <Fem3dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::FEM3D_EXAMPLE_TEXT).expect("default example snapshot");
    let config = Fem3dResultsWindowConfig::default();
    let node = render(&snapshot, &config, &Fem3dInteractionSnapshot::default(), None, None).expect("fixture surface admission");
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
    let config = Fem3dResultsWindowConfig { result_source_id: Some("dead".into()), result_mode: semio_s_fem_app_surface::ResultMode::Static, ..Fem3dResultsWindowConfig::default() };
    let node = render(&snapshot, &config, &Fem3dInteractionSnapshot::default(), None, None).expect("fixture surface admission");
    let scene_stack = node.children.get(1).expect("caption + growing scene stack");
    let surface = scene_stack.children.iter().find(|child| matches!(&child.component, semio_framework_ui_contract::Component::Surface(_))).expect("world surface child");
    let scene: semio_framework_ui_scene::World3dScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(surface).expect("assemble world scene");
    assert!(scene.meshes_json.contains("solid-sol1"), "expected the solid mesh in the results scene: {}", scene.meshes_json);
    assert!(scene.meshes_json.contains("\"colors\""), "expected a vertex colors array on the solid mesh data: {}", scene.meshes_json);
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("fixture projection");
    assert!(json.contains("Case: dead"), "expected a case-id caption: {json}");
}

#[semio_framework_async_macros::async_test]
async fn results_scene_captions_name_mode_and_factor_3d() {
    let mut app = app_with_example().await;
    dispatch(&mut app, Fem3dCommand::SetResultDisplay(semio_s_artifact_fem_3d_editor::editor::fem3d::commands::set_result_display::SetResultDisplay { source_id: None, mode: "modal".into(), mode_index: 0, field: None, value: None, window_id: None })).await;
    let json_modal = render_body(&mut app, FEM3D_BODY_RESULTS);
    assert!(json_modal.contains("Hz"), "expected a frequency caption: {json_modal}");

    dispatch(&mut app, Fem3dCommand::SetResultDisplay(semio_s_artifact_fem_3d_editor::editor::fem3d::commands::set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "buckling".into(), mode_index: 0, field: None, value: None, window_id: None })).await;
    let json_buckling = render_body(&mut app, FEM3D_BODY_RESULTS);
    assert!(json_buckling.contains("factor"), "expected a load-factor caption: {json_buckling}");
}

#[semio_framework_async_macros::async_test]
async fn fem3d_model_extent_degenerate_model_returns_one() {
    assert_eq!(fem3d_model_extent(&Fem3dSnapshot::default()), 1.0);
}

/// 🧵️ Mounted rendering publishes pending scenes without executing a numerical solve.
#[semio_framework_async_macros::async_test]
async fn mounted_results_render_borrows_jobs_without_synchronous_solves() {
    let doc = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::fem3d_demo_snapshot();
    reset_results_cache();
    let grant = semio_framework_value::RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 32768, maximum_capacity_bytes: 262144, maximum_release_bytes: 1048576, maximum_depth: 4096 };
    let operation = semio_framework_plugin::AppRenderOperationContext { app_instance_id: 107, base_revision: semio_framework_job::RevisionId(1), generation: semio_framework_job::Generation(1), canonical_base_revision: [1; 32], mounted_policy: semio_framework_plugin::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } };
    for revision in [operation, semio_framework_plugin::AppRenderOperationContext { canonical_base_revision: [2; 32], ..operation }] {
        let node = render(&doc, &Fem3dResultsWindowConfig::default(), &Fem3dInteractionSnapshot::default(), Some(revision), Some("fixture-results")).unwrap();
        let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).unwrap();
        assert!(json.contains("Analysis running"));
    }
    eprintln!("[DEBUG] FEM3D mounted results revisions render with {} synchronous solver calls", results_solve_count());
    assert_eq!(results_solve_count(), 0);
}
