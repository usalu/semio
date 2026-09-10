//! 👁️ Generation3d play app — the generation output-preview window (generate mode): a tessellated
//! preview of the patched fixture's evaluated geometry.

use crate::editor::generation3d::config::Generation3dConfig;
use crate::editor::generation3d::modes::edit::windows::preview::show_mode_measure;
use crate::editor::generation3d::terminology::Generation3dLabels;
use crate::editor::generation3d::GENERATION_3D_PLAY_APP_ID;
use crate::editor::generation3d::{preview_camera_json, preview_payload, preview_selection_json, PreviewInteractionMarks, PreviewPayload, GENERATION_3D_INTERACTION_DOMAIN, GENERATION_3D_INTERACTION_GRANULARITY};
use crate::standards::v1::subsets::any::schema::generation_fixture_for;
use semio_framework_artifact_flow_flow::FlowFixture;
use semio_framework_artifact_playbook_playbook::{selected_generation, GenerationPlayState};
use semio_framework_plugin::{world3d_scene, world3d_sun_measures, BuiltNode, LocalizedLabel, SurfaceKind, TextEditorScene, WindowKindDefinition, WindowMeasure, WindowOptions};

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

//#region 🔖️Render
pub fn render(
    fixture: &FlowFixture,
    generation: &GenerationPlayState,
    generation_preview_text: Option<&str>,
    cfg: &Generation3dConfig,
    labels: &Generation3dLabels,
    active_utility: &str,
    marks: &PreviewInteractionMarks,
) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let payload = match selected_generation(generation) {
        // 🧹️ `generation_fixture_for` CLONES the document fixture, so the patched copy owns its own
        // `layout` ordered-map root and must be retired before it leaves scope — a bare drop aborts the
        // plugin actor with `ordered-map root must be explicitly retired before drop`. Reachable only
        // once a generation is selected, which is why no earlier render test ever hit it
        // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        Some(_) => {
            let gen_fixture = generation_fixture_for(fixture, generation);
            let payload = preview_payload(generation_preview_text.unwrap_or_default(), &gen_fixture, cfg, None, marks);
            gen_fixture.retire_cold();
            payload
        }
        None => PreviewPayload::default(),
    };
    if payload.meshes_json == "[]" && payload.instances_json == "[]" {
        let text = generation_preview_text.filter(|value| !value.is_empty()).unwrap_or(labels.preview_hint.as_str());
        let scene = TextEditorScene::base(text.to_string(), Some("json".into()), None);
        return crate::scene_surface(GENERATION_3D_PLAY_SURFACE_GENERATE_PREVIEW, semio_framework_plugin::plugin_app_close_prelude::SurfaceKind::TextEditor, &scene);
    }
    let sun = cfg.sun();
    let selection_json = preview_selection_json(cfg, active_utility, &payload);
    let _ = GENERATION_3D_PLAY_APP_ID;
    crate::scene_surface(
        GENERATION_3D_PLAY_SURFACE_GENERATE_PREVIEW,
        semio_framework_plugin::plugin_app_close_prelude::SurfaceKind::World3d,
        &semio_framework_ui::wgpu::World3dScene {
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
