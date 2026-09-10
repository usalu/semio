use super::*;
use crate::editor::generation3d::commands::set_active_example;
use crate::editor::generation3d::testkit::{app_with_registry, dispatch, drain_flow_eval_ticks, render as render_body};
use crate::editor::generation3d::Generation3dCommand;
use crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_BOX_FILLET;

/// 🚚️ The ASSEMBLED world-3d scene a render host reads back out of the projected preview body.
/// Since the paged-scene wave (26/09/02 P) `meshesJson`/`instancesJson`/`cameraJson` do NOT ride
/// inside the surface's fixed-capacity `doc` at all — `SceneDoc::split_lanes` moves each one into
/// its own `paged_text_carrier` child — so scanning the projected tree for a string field named
/// `meshesJson` finds nothing. `decode_fixture_scene_with_lanes` is the framework's own inverse
/// (the Rust twin of the React Interpreter's `world3dSceneFromLanes`).
fn preview_scene(projection: &str) -> semio_framework_ui::wgpu::World3dScene {
    semio_framework_plugin::testkit::decode_fixture_scene_with_lanes(projection).expect("projected preview body must decode as an assembled world-3d scene")
}

#[semio_framework_async_macros::async_test]
async fn renders_world_preview_scene() {
    // 🧵️ Rendering the preview body tessellates BRep geometry through the same process-wide cache
    // `apps::generation3d`'s own tests serialize on — see that module's `test_support`.
    let _serial = crate::editor::generation3d::test_support::lock();
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
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app_with_registry().await;
    drain_flow_eval_ticks(&mut app).await;
    let before_meshes = preview_scene(&render_body(&mut app, GENERATION_3D_PLAY_BODY_PREVIEW).await).meshes_json;
    dispatch(&mut app, Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: PROCEDURAL_EXAMPLE_BOX_FILLET.into() })).await;
    drain_flow_eval_ticks(&mut app).await;
    let after = preview_scene(&render_body(&mut app, GENERATION_3D_PLAY_BODY_PREVIEW).await);
    assert_ne!(after.meshes_json, "[]", "box-fillet-preview must tessellate into non-empty preview meshes");
    assert_ne!(after.instances_json, "[]", "box-fillet-preview must produce non-empty preview instances");
    assert_ne!(after.meshes_json, before_meshes, "switching active example must change the tessellated preview meshes");
}
