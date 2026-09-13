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

/// ⚖️ LAW: the two World3d previews offer the SAME direct-manipulation surface. Both declare the
/// `move`/`rotate`/`scale` transform utilities and both own the three gumball verbs, because both
/// paint the same instances over the same `graph` interaction domain through the same
/// `preview_selection_json` (whose `transformMode`/`gumballActive` pair is what `World3dHost` gates
/// the gumball on).
///
/// 🐛️ The generate preview used to declare neither, so `ViewModel.active_utility_id` never became a
/// transform mode there and a user who selected a generated instance had no way to move it — a
/// capability gap between two windows that otherwise look identical
/// (`📓️audit-user-journey-gaps-2026-09-13.md` gap #5).
#[test]
fn both_previews_offer_the_same_gumball_surface() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let definition = crate::editor::generation3d::create_generation3d_app();
    let window = |id: &str| definition.window_kinds.iter().find(|kind| kind.id == id).unwrap_or_else(|| panic!("{id} window kind"));
    let actions = |id: &str| window(id).actions.iter().map(|action| action.id.clone()).collect::<std::collections::BTreeSet<_>>();
    let utilities = |id: &str| window(id).utilities.iter().map(|utility| utility.as_str().to_string()).collect::<Vec<_>>();
    let edit = crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW;
    let generate = GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW;
    for verb in ["translateSelection", "rotateSelection", "scaleSelection"] {
        assert!(actions(generate).contains(verb), "the generate preview must own {verb}, the edit preview already does");
    }
    assert_eq!(utilities(generate), utilities(edit), "both previews must offer the same transform utility rail");
    assert_eq!(utilities(generate), vec!["move".to_string(), "rotate".to_string(), "scale".to_string()]);
    eprintln!("[DEBUG] generate preview actions={:?} utilities={:?}", actions(generate), utilities(generate));
}

/// ⚖️ LAW: the generate preview's selection payload carries a live gumball as soon as something is
/// selected under a transform utility — the exact predicate `World3dHost` reads
/// (`gumballVisible = selection.gumballActive && transformGumballMode`).
#[test]
fn the_generate_preview_selection_payload_arms_the_gumball_under_a_transform_utility() {
    let config = crate::editor::generation3d::config::Generation3dConfig::default();
    let payload = crate::editor::generation3d::PreviewPayload { selected_ids: vec!["extrude@solid#0".into()], ..Default::default() };
    let armed: serde_json::Value = serde_json::from_str(&crate::editor::generation3d::preview_selection_json(&config, "move", &payload)).expect("selection json");
    assert_eq!(armed["gumballActive"], serde_json::Value::Bool(true));
    assert_eq!(armed["transformMode"], serde_json::Value::String("move".into()));
    let idle: serde_json::Value = serde_json::from_str(&crate::editor::generation3d::preview_selection_json(&config, "move", &crate::editor::generation3d::PreviewPayload::default())).expect("selection json");
    assert_eq!(idle["gumballActive"], serde_json::Value::Bool(false), "an empty selection must never arm a gumball");
}
