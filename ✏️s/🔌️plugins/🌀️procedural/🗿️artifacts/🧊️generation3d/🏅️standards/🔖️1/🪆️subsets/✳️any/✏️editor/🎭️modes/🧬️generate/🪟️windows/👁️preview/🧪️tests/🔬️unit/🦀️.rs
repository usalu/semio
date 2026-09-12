use super::*;
use crate::editor::generation3d::unit_tests::context::{app, render as render_body};
use semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes;

/// ⚖️ LAW: the generate-mode ENTRY state. Before any generation exists the preview window is already a
/// WORLD-3D status host — empty meshes, a live tessellation `phase`, and the authored hint — never a
/// different surface kind.
///
/// 🐛️ The old text-editor fallback published no `[data-status-json]` host at all, so entering generate
/// mode left the playground with `hosts=[]` and nothing a progress/idle state could ever be read from
/// (`🗑️generated/journey-3/results.json`, 189 s of generate mode with no host; ticket
/// 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn generate_preview_entry_state_is_an_idle_status_host_with_a_hint() {
    let mut app = app().await;
    let body = render_body(&mut app, GENERATION_3D_PLAY_BODY_GENERATE_PREVIEW).await;
    let scene: semio_framework_ui::wgpu::World3dScene = decode_fixture_scene_with_lanes(&body).expect("the generate-mode entry preview is a world-3d surface");
    assert_eq!(scene.meshes_json, "[]", "nothing is evaluated yet: {}", scene.meshes_json);
    let status_json = scene.status_json.clone().expect("the generate preview publishes a status host from its first frame");
    let status: serde_json::Value = serde_json::from_str(&status_json).expect("generate preview status json");
    assert!(status.get("phase").and_then(serde_json::Value::as_str).is_some(), "the entry status must carry a tessellation phase: {status_json}");
    assert!(status.get("progress").is_some(), "the entry status must carry progress counters: {status_json}");
    let hint = status.get("hint").and_then(serde_json::Value::as_str).unwrap_or_default();
    assert!(hint.contains("evaluate a generation"), "the entry status must hint at creating a generation: {status_json}");
    eprintln!("[DEBUG] generate preview entry status={status_json}");
}
