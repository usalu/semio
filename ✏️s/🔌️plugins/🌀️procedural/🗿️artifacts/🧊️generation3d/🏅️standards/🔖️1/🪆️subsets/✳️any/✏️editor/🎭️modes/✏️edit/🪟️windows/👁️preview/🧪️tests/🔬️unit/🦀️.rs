
use super::*;
use crate::editor::generation3d::Generation3dCommand;
use crate::editor::generation3d::commands::set_active_example;
use crate::editor::generation3d::testkit::{app_with_registry, dispatch, drain_flow_eval_ticks, render as render_body};
use crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_BOX_FILLET;

/// 🔎️ Recursively finds a string-valued JSON field named `key` anywhere in `value` — needed
/// because `scene_surface` may nest the `World3dScene` fields at an arbitrary depth inside the
/// rendered `ComponentTree`.
fn find_json_string_field<'a>(value: &'a serde_json::Value, key: &str) -> Option<&'a str> {
    match value {
        serde_json::Value::Object(map) => {
            if let Some(serde_json::Value::String(found)) = map.get(key) {
                return Some(found.as_str());
            }
            map.values().find_map(|entry| find_json_string_field(entry, key))
        }
        serde_json::Value::Array(items) => items.iter().find_map(|entry| find_json_string_field(entry, key)),
        _ => None,
    }
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
    let value: serde_json::Value = serde_json::from_str(&json).expect("preview render must be valid json");
    let meshes_json = find_json_string_field(&value, "meshesJson").expect("world-3d scene must carry a meshesJson field");
    let instances_json = find_json_string_field(&value, "instancesJson").expect("world-3d scene must carry an instancesJson field");
    assert_ne!(meshes_json, "[]", "hexagonal-mushroom-column must tessellate into non-empty preview meshes");
    assert_ne!(instances_json, "[]", "hexagonal-mushroom-column must produce non-empty preview instances");
}

/// 🔁️ Drives `setActiveExample` through the real dispatch path (registry-backed, so the
/// `Generation3dBoundedCommandJobFactory` classification actually runs) and proves the preview
/// re-tessellates: meshes stay non-empty and differ from the boot fixture's own meshes.
#[semio_framework_async_macros::async_test]
async fn switching_active_example_changes_preview_meshes() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app_with_registry().await;
    drain_flow_eval_ticks(&mut app).await;
    let before_json = render_body(&mut app, GENERATION_3D_PLAY_BODY_PREVIEW).await;
    let before_value: serde_json::Value = serde_json::from_str(&before_json).expect("preview render must be valid json");
    let before_meshes = find_json_string_field(&before_value, "meshesJson").expect("world-3d scene must carry a meshesJson field").to_string();
    dispatch(&mut app, Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: PROCEDURAL_EXAMPLE_BOX_FILLET.into() })).await;
    drain_flow_eval_ticks(&mut app).await;
    let after_json = render_body(&mut app, GENERATION_3D_PLAY_BODY_PREVIEW).await;
    let after_value: serde_json::Value = serde_json::from_str(&after_json).expect("preview render must be valid json");
    let after_meshes = find_json_string_field(&after_value, "meshesJson").expect("world-3d scene must carry a meshesJson field");
    let after_instances = find_json_string_field(&after_value, "instancesJson").expect("world-3d scene must carry an instancesJson field");
    assert_ne!(after_meshes, "[]", "box-fillet-preview must tessellate into non-empty preview meshes");
    assert_ne!(after_instances, "[]", "box-fillet-preview must produce non-empty preview instances");
    assert_ne!(after_meshes, before_meshes.as_str(), "switching active example must change the tessellated preview meshes");
}
