//! 👁️ Procedural3d play app — the generation output-preview window (generate mode): a tessellated
//! preview of the patched fixture's evaluated geometry.

use crate::editor::procedural3d::config::Procedural3dConfig;
use crate::editor::procedural3d::modes::edit::windows::preview::show_mode_measure;
use crate::editor::procedural3d::terminology::Procedural3dLabels;
use crate::editor::procedural3d::PROCEDURAL_3D_PLAY_APP_ID;
use crate::artifacts::procedural3d::schema::generation_fixture_for;
use crate::editor::procedural3d::{preview_camera_json, preview_payload_from_eval, preview_selection_json};
use flow::FlowFixture;
use flow::playbook::{render_generation_preview_text, selected_generation, GenerationPlayState};
use semio_framework_plugin::{build_world_3d_scene, world3d_scene, world3d_sun_measures, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowMeasure, WindowOptions};

//#region 🔖️Constants
pub const PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW: &str = "procedural3d-generate-preview";
pub const PROCEDURAL_3D_PLAY_BODY_GENERATE_PREVIEW: &str = "procedural.play.generate-preview";
const PROCEDURAL_3D_PLAY_SURFACE_GENERATE_PREVIEW: &str = "procedural.play.generate-preview";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW.into(),
        label: LocalizedLabel::native("Preview", "Vorschau"),
        body_key: PROCEDURAL_3D_PLAY_BODY_GENERATE_PREVIEW.into(),
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
        capabilities: Vec::new()}
}

/// 🎚️ Shares the same show-mode + sun measures as the edit-mode 3D preview window.
pub fn window_measures(config: &Procedural3dConfig, procedural_action: impl Fn(&str, Option<serde_json::Value>) -> semio_framework_plugin::ActionDescriptor + Copy) -> Vec<WindowMeasure> {
    let sun = config.sun();
    vec![show_mode_measure(&config.show_mode, procedural_action), world3d_sun_measures("procedural3d", &sun, procedural_action)]
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(fixture: &FlowFixture, generation: &GenerationPlayState, cfg: &Procedural3dConfig, labels: &Procedural3dLabels, active_utility: &str) -> UiNode {
    let (meshes_json, instances_json) = match selected_generation(generation) {
        Some(_) => {
            let gen_fixture = generation_fixture_for(fixture, generation);
            let eval_json = generation.preview_text.clone().unwrap_or_default();
            preview_payload_from_eval(&eval_json, &gen_fixture, cfg)
        }
        None => ("[]".into(), "[]".into())};
    if meshes_json == "[]" && instances_json == "[]" {
        let text = generation.preview_text.as_deref().filter(|value| !value.is_empty()).unwrap_or(labels.preview_hint.as_str());
        return render_generation_preview_text(PROCEDURAL_3D_PLAY_SURFACE_GENERATE_PREVIEW, PROCEDURAL_3D_PLAY_APP_ID, text);
    }
    let sun = cfg.sun();
    build_world_3d_scene(PROCEDURAL_3D_PLAY_SURFACE_GENERATE_PREVIEW, PROCEDURAL_3D_PLAY_APP_ID, world3d_scene(preview_camera_json(cfg), meshes_json, instances_json, preview_selection_json(cfg, active_utility), &sun))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedural3d::testkit::{app, render as render_body};

    #[test]
    fn generate_preview_hints_without_evaluated_output() {
        let mut app = app();
        assert!(render_body(&mut app, PROCEDURAL_3D_PLAY_BODY_GENERATE_PREVIEW).contains("evaluate a generation"));
    }
}
//#endregion 🧪️Tests
