use super::*;
use crate::editor::generation3d::commands::set_active_example;
use crate::editor::generation3d::unit_tests::context::{self, app_with_registry, drain_flow_eval_ticks, render as render_body};
use crate::editor::generation3d::Generation3dCommand;
use crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_BOX_FILLET;
use semio_framework_plugin::PluginApp;

/// 🚚️ The ASSEMBLED world-3d scene a render host reads back out of the projected preview body.
/// Since the paged-scene wave (26/09/02 P) `meshesJson`/`instancesJson`/`cameraJson` do NOT ride
/// inside the surface's fixed-capacity `doc` at all — `SceneDoc::split_lanes` moves each one into
/// its own `paged_text_carrier` child — so scanning the projected tree for a string field named
/// `meshesJson` finds nothing. `decode_fixture_scene_with_lanes` is the framework's own inverse
/// (the Rust twin of the React Interpreter's `world3dSceneFromLanes`).
fn preview_scene(projection: &str) -> semio_framework_ui::wgpu::World3dScene {
    semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(projection).expect("projected preview body must decode as an assembled world-3d scene")
}

#[semio_framework_async_macros::async_test]
async fn renders_world_preview_scene() {
    // 🧵️ Rendering the preview body tessellates BRep geometry through the same process-wide cache
    // `apps::generation3d`'s own tests serialize on — see that module's `unit_tests::serial_execution`.
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    drain_flow_eval_ticks(&mut app).await;
    let json = render_body(&mut app, GENERATION_3D_PLAY_BODY_PREVIEW).await;
    assert!(json.contains("world-3d"));
    // 🐛️ Regression guard for the empty-scene defect: `handle`/`render` used to construct a
    // brand-new `FlowEvalSession` on every call, so `eval_json` was always `""` and
    // `preview_payload` short-circuited to `("[]", "[]")` despite the default
    // `hexagonal-mushroom-column` fixture being non-empty. `json.contains("world-3d")` alone
    // never caught this — both fields still assert non-empty below.
    let scene = preview_scene(&json);
    assert_ne!(scene.meshes_json, "[]", "hexagonal-mushroom-column must tessellate into non-empty preview meshes");
    assert_ne!(scene.instances_json, "[]", "hexagonal-mushroom-column must produce non-empty preview instances");
}

/// 🔁️ Drives `setActiveExample` through the real dispatch path (registry-backed, so the
/// `Generation3dBoundedCommandJobFactory` classification actually runs) and proves the preview
/// re-tessellates: meshes stay non-empty and differ from the boot fixture's own meshes.
#[semio_framework_async_macros::async_test]
async fn switching_active_example_changes_preview_meshes() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    drain_flow_eval_ticks(&mut app).await;
    let before_meshes = preview_scene(&render_body(&mut app, GENERATION_3D_PLAY_BODY_PREVIEW).await).meshes_json;
    let (view, _) = context::preview_views("procedural-preview-test", "procedural-preview-test-other");
    let switched = context::dispatch_with_view(&mut app, Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: PROCEDURAL_EXAMPLE_BOX_FILLET.into() }), view.clone()).await.expect("the switch dispatches under the preview window");
    context::drive_preview_run(&mut app, &view, &switched.effects).await;
    let after = preview_scene(&render_body(&mut app, GENERATION_3D_PLAY_BODY_PREVIEW).await);
    assert_ne!(after.meshes_json, "[]", "box-fillet-preview must tessellate into non-empty preview meshes");
    assert_ne!(after.instances_json, "[]", "box-fillet-preview must produce non-empty preview instances");
    assert_ne!(after.meshes_json, before_meshes, "switching active example must change the tessellated preview meshes");
}

/// ⚖️ LAW: the DEMONSTRATOR's own boot path — the generator pane's `ShellHost` dispatches
/// `setActiveExample` with the brand's default example (`🪧️brand.ts`
/// `ENTWERFEN_MIT_BESTAND_GENERATOR_BRAND.defaults.exampleId`) under the FLOW window, exactly as
/// `ShellHost/🟦️.tsx`'s boot effect does, and the `previewEval` run that gesture starts must leave
/// the PREVIEW window's rendered world-3d scene carrying real geometry.
///
/// 🐛️ `📓️app-generator.md` §3 recorded this as a structural gap — `render()` building a fresh,
/// never-ticked `FlowEvalSession` per call — and the demonstrator acceptance suite still carries that
/// note. The session is retained per app instance now and the evaluation is a progress/cancel-capable
/// `previewEval` run, so this law is what states the boot contract in the surface's own terms: one
/// gesture, one run driven to completion, meshes and instances on the window it addressed. It reads
/// the rendered SCENE rather than the session, because an evaluation nobody paints is the very defect.
#[semio_framework_async_macros::async_test]
async fn the_demonstrator_boot_example_renders_a_non_empty_preview_scene() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    let (flow_view, preview_view) = context::shell_views(crate::editor::generation3d::modes::edit::windows::flow::GENERATION_3D_PLAY_WINDOW_MAIN, GENERATION_3D_PLAY_WINDOW_PREVIEW);
    let action_meta = semio_framework_plugin::ActionMeta { view_state: Some(flow_view.clone()), ..semio_framework_plugin::artifact_app_laws::meta("local") };
    app.handle_action("setActiveExample", Some(&serde_json::json!({ "exampleId": crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_HEX_COLUMN }).into()), &action_meta)
        .await
        .expect("the generator pane's boot dispatches setActiveExample under the flow window");
    let receipt = context::settle(&mut app).await;
    assert!(!receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Fault), "the boot example switch published a fault lane");
    let run = context::drive_preview_run(&mut app, &flow_view, &receipt.effects).await;
    eprintln!("[DEBUG] demonstrator boot run: hops={} windows={:?} answered={} state={:?}", run.hops, run.hop_windows, run.answered, run.state);
    assert!(run.hop_windows.iter().any(|window| window == GENERATION_3D_PLAY_WINDOW_PREVIEW), "the boot run never evaluated the preview window, got {:?}", run.hop_windows);
    let scene = preview_scene(&context::render_with_view(&mut app, GENERATION_3D_PLAY_BODY_PREVIEW, &preview_view).await);
    assert_ne!(scene.meshes_json, "[]", "the demonstrator's boot example must paint non-empty preview meshes");
    assert_ne!(scene.instances_json, "[]", "the demonstrator's boot example must paint non-empty preview instances");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

/// ⚖️ LAW: the render is a pure READ of the retained evaluation — it never evaluates anything of its
/// own. Before the run has taken a hop the preview paints nothing, and two renders after it are
/// byte-identical, which is what makes the evaluation cancellable at all: a surface that re-evaluated
/// per render would rebuild the geometry a user just aborted (`📓️app-generator.md` §7 fix 1).
#[semio_framework_async_macros::async_test]
async fn the_preview_render_reads_the_retained_evaluation_instead_of_recomputing_it() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    let (flow_view, preview_view) = context::shell_views(crate::editor::generation3d::modes::edit::windows::flow::GENERATION_3D_PLAY_WINDOW_MAIN, GENERATION_3D_PLAY_WINDOW_PREVIEW);
    let unevaluated = preview_scene(&context::render_with_view(&mut app, GENERATION_3D_PLAY_BODY_PREVIEW, &preview_view).await);
    assert_eq!(unevaluated.meshes_json, "[]", "a render before the run's first hop must paint nothing, not evaluate the graph itself");
    context::drive_preview_run(&mut app, &flow_view, &[]).await;
    let first = preview_scene(&context::render_with_view(&mut app, GENERATION_3D_PLAY_BODY_PREVIEW, &preview_view).await);
    let second = preview_scene(&context::render_with_view(&mut app, GENERATION_3D_PLAY_BODY_PREVIEW, &preview_view).await);
    assert_ne!(first.meshes_json, "[]", "the driven run must leave the preview carrying meshes");
    assert_eq!(first.meshes_json, second.meshes_json, "two renders of one retained evaluation must publish the same meshes");
    assert_eq!(first.instances_json, second.instances_json, "two renders of one retained evaluation must publish the same instances");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}
