//! 👁️ Generation3d play app — the generation output-preview window (generate mode): a tessellated
//! preview of the patched fixture's evaluated geometry.

use crate::editor::generation3d::config::Generation3dConfig;
use crate::editor::generation3d::modes::edit::windows::preview::show_mode_measure;
use crate::editor::generation3d::terminology::Generation3dLabels;
use crate::editor::generation3d::GENERATION_3D_PLAY_APP_ID;
use crate::editor::generation3d::{preview_camera_json, preview_payload, preview_selection_json, preview_status_json, preview_window_status_json, PreviewInteractionMarks, PreviewPayload, PreviewStatusDebug, GENERATION_3D_INTERACTION_DOMAIN, GENERATION_3D_INTERACTION_GRANULARITY};
use crate::standards::v1::subsets::any::schema::generation_fixture_for;
use semio_framework_artifact_flow_flow::FlowFixture;
use semio_framework_artifact_playbook_playbook::{selected_generation, GenerationPlayState};
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{world3d_scene, world3d_sun_measures, BuiltNode, LocalizedLabel, SurfaceKind, WindowKindDefinition, WindowMeasure, WindowOptions};

#[path = "🫧️transient/🦀️.rs"]
pub mod transient;

//#region 🔖️Constants
pub const GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW: &str = "generation3d-generate-preview";
pub const GENERATION_3D_PLAY_BODY_GENERATE_PREVIEW: &str = "procedural.play.generate-preview";
const GENERATION_3D_PLAY_SURFACE_GENERATE_PREVIEW: &str = "procedural.play.generate-preview";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW.into(),
        label: LocalizedLabel::native("Preview", "Vorschau"),
        body_key: GENERATION_3D_PLAY_BODY_GENERATE_PREVIEW.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "preview".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🎚️ Shares the same show-mode + sun measures as the edit-mode 3D preview window.
pub fn window_measures(config: &Generation3dConfig, procedural_action: impl Fn(&str, Option<serde_json::Value>) -> semio_framework_plugin::ActionDescriptor + Copy) -> Vec<WindowMeasure> {
    let sun = config.sun();
    vec![show_mode_measure(&config.show_mode, procedural_action), world3d_sun_measures("generation3d", &sun, procedural_action)]
}
//#endregion 🔖️Definition

/// 📈 The ONE status object this window publishes, in every state — the generate-mode ENTRY state
/// included. It carries the session's tessellation `phase`/`progress` (so the shell's world host can
/// show and cancel an evaluation the same way edit-mode preview does), the `debug` counters the
/// browser probe reads back off `data-status-json`, and — only while there is nothing to show — the
/// authored `hint`.
///
/// 🐛️ Why the window is a world host even with nothing evaluated: falling back to a `TextEditor`
/// surface published NO status host at all, so entering generate mode left the playground with
/// `hosts=[]` and no window a status could ever appear on — measured on 6018 as 189 s of
/// `[data-status-json]`-less generate mode (`🗑️generated/journey-3/results.json`, ticket
/// 26/09/09/PROCEDURAL-3D-END-TO-END). A hint is a state of this window, not a different window.
fn generate_preview_status_json(session: &FlowEvalSession, eval_json: &str, payload: &PreviewPayload, preview_status: Option<String>, hint: Option<&str>) -> Option<String> {
    preview_window_status_json(Some(session), preview_status, &PreviewStatusDebug { eval_json, meshes_json: &payload.meshes_json, instances_json: &payload.instances_json }, hint)
}

//#region 🔖️Render
pub fn render(
    fixture: &FlowFixture,
    generation: &GenerationPlayState,
    generation_preview_text: Option<&str>,
    cfg: &Generation3dConfig,
    labels: &Generation3dLabels,
    active_utility: &str,
    marks: &PreviewInteractionMarks,
    session: &FlowEvalSession,
) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let eval_json = generation_preview_text.unwrap_or_default();
    let (payload, preview_status) = match selected_generation(generation) {
        // 🧹️ `generation_fixture_for` CLONES the document fixture, so the patched copy owns its own
        // `layout` ordered-map root and must be retired before it leaves scope — a bare drop aborts the
        // plugin actor with `ordered-map root must be explicitly retired before drop`. Reachable only
        // once a generation is selected, which is why no earlier render test ever hit it
        // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        Some(_) => {
            let gen_fixture = generation_fixture_for(fixture, generation);
            let payload = preview_payload(eval_json, &gen_fixture, cfg, Some(session), marks);
            let preview_status = preview_status_json(eval_json, &gen_fixture);
            gen_fixture.retire_cold();
            (payload, preview_status)
        }
        None => (PreviewPayload::default(), None),
    };
    let empty = payload.meshes_json == "[]" && payload.instances_json == "[]";
    let status_json = generate_preview_status_json(session, eval_json, &payload, preview_status, empty.then(|| labels.preview_hint.as_str()));
    let sun = cfg.sun();
    let selection_json = preview_selection_json(cfg, active_utility, &payload);
    let _ = GENERATION_3D_PLAY_APP_ID;
    crate::scene_surface(
        GENERATION_3D_PLAY_SURFACE_GENERATE_PREVIEW,
        semio_framework_plugin::plugin_app_close_prelude::SurfaceKind::World3d,
        &semio_framework_ui::wgpu::World3dScene {
            status_json,
            domain_id: Some(GENERATION_3D_INTERACTION_DOMAIN.into()),
            domain_granularity_id: Some(GENERATION_3D_INTERACTION_GRANULARITY.into()),
            ..world3d_scene(preview_camera_json(cfg), payload.meshes_json, payload.instances_json, selection_json, &sun)
        },
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "../../🧪️tests/👁️preview/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
