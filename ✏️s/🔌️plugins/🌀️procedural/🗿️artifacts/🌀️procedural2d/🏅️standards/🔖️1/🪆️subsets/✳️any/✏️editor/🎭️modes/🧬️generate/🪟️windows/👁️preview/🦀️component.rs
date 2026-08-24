//! 👁️ Procedural2d play app — the generation output-preview window (generate mode).

use crate::artifacts::procedural2d::schema::generation_preview_layers;
use crate::editor::procedural2d::config::Procedural2dConfig;
use crate::editor::procedural2d::terminology::Procedural2dLabels;
use crate::editor::procedural2d::PROCEDURAL2D_PLAY_APP_ID;
use semio_framework_plugin::{built_text_node, BuiltNode, Canvas2dScene, LocalizedLabel, SurfaceKind, TextEditorScene, WindowKindDefinition, WindowOptions};

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
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(config: &Procedural2dConfig, labels: &Procedural2dLabels) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let eval_json = config.generation_preview_text.as_deref().filter(|value| !value.is_empty()).unwrap_or("");
    if eval_json.is_empty() {
        return built_text_node(semio_framework_plugin::Label::data(labels.preview_hint.as_str()))
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.generate-preview.hint", "fixed UI hint admission failed"));
    }
    let layers = generation_preview_layers(eval_json);
    if layers == "[]" {
        let scene = TextEditorScene::base(eval_json.to_string(), Some("json".into()), None);
        return crate::scene_surface(PROCEDURAL2D_PLAY_SURFACE_GENERATE_PREVIEW, semio_framework_plugin::plugin_app_close_prelude::SurfaceKind::TextEditor, &scene);
    }
    let _ = PROCEDURAL2D_PLAY_APP_ID;
    crate::scene_surface(
        PROCEDURAL2D_PLAY_SURFACE_GENERATE_PREVIEW,
        semio_framework_plugin::plugin_app_close_prelude::SurfaceKind::Canvas2d,
        &Canvas2dScene { camera_x: config.camera.x, camera_y: config.camera.y, zoom: config.camera.zoom, layers_json: layers, snapshot: None },
    )
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
