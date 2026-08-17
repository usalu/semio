//! 👁️ Procedural2d play app — the generation output-preview window (generate mode).

use crate::editor::procedural2d::config::Procedural2dConfig;
use crate::editor::procedural2d::terminology::Procedural2dLabels;
use crate::editor::procedural2d::PROCEDURAL2D_PLAY_APP_ID;
use crate::artifacts::procedural2d::schema::generation_preview_layers;
use flow::playbook::render_generation_preview_text;
use semio_framework_plugin::{build_canvas_2d_scene, Canvas2dScene, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const PROCEDURAL2D_PLAY_WINDOW_GENERATE_PREVIEW: &str = "procedural2d-generate-preview";
pub const PROCEDURAL2D_PLAY_BODY_GENERATE_PREVIEW: &str = "procedural2d.play.generate-preview";
const PROCEDURAL2D_PLAY_SURFACE_GENERATE_PREVIEW: &str = "procedural2d.play.generate-preview";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: PROCEDURAL2D_PLAY_WINDOW_GENERATE_PREVIEW.into(),
        label: LocalizedLabel::native("Preview", "Vorschau"),
        body_key: PROCEDURAL2D_PLAY_BODY_GENERATE_PREVIEW.into(),
        surface_kind: SurfaceKind::Canvas2d,
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
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(config: &Procedural2dConfig, labels: &Procedural2dLabels) -> UiNode {
    let eval_json = config.generation_preview_text.as_deref().filter(|value| !value.is_empty()).unwrap_or("");
    if eval_json.is_empty() {
        return semio_framework_plugin::ui_text(labels.preview_hint);
    }
    let layers = generation_preview_layers(eval_json);
    if layers == "[]" {
        return render_generation_preview_text(PROCEDURAL2D_PLAY_SURFACE_GENERATE_PREVIEW, PROCEDURAL2D_PLAY_APP_ID, eval_json);
    }
    build_canvas_2d_scene(PROCEDURAL2D_PLAY_SURFACE_GENERATE_PREVIEW, PROCEDURAL2D_PLAY_APP_ID, Canvas2dScene { camera_x: config.camera.x, camera_y: config.camera.y, zoom: config.camera.zoom, layers_json: layers })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedural2d::testkit::{app, render as render_body};

    #[test]
    fn generate_preview_hints_without_evaluated_output() {
        let mut app = app();
        assert!(render_body(&mut app, PROCEDURAL2D_PLAY_BODY_GENERATE_PREVIEW).contains("evaluate a generation"));
    }
}
//#endregion 🧪️Tests
