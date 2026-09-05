//! 👁️ Generation2d play app — the generation output-preview window (generate mode).

use crate::artifacts::generation2d::schema::generation_preview_layers;
use crate::editor::generation2d::config::Generation2dConfig;
use crate::editor::generation2d::terminology::Generation2dLabels;
use crate::editor::generation2d::GENERATION2D_PLAY_APP_ID;
use semio_framework_plugin::{built_text_node, BuiltNode, Canvas2dScene, LocalizedLabel, SurfaceKind, TextEditorScene, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const GENERATION2D_PLAY_WINDOW_GENERATE_PREVIEW: &str = "generation2d-generate-preview";
pub const GENERATION2D_PLAY_BODY_GENERATE_PREVIEW: &str = "generation2d.play.generate-preview";
const GENERATION2D_PLAY_SURFACE_GENERATE_PREVIEW: &str = "generation2d.play.generate-preview";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: GENERATION2D_PLAY_WINDOW_GENERATE_PREVIEW.into(),
        label: LocalizedLabel::native("Preview", "Vorschau"),
        body_key: GENERATION2D_PLAY_BODY_GENERATE_PREVIEW.into(),
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
pub fn render(config: &Generation2dConfig, labels: &Generation2dLabels) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let eval_json = config.generation_preview_text.as_deref().filter(|value| !value.is_empty()).unwrap_or("");
    if eval_json.is_empty() {
        return built_text_node(semio_framework_plugin::Label::data(labels.preview_hint.as_str())).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.generate-preview.hint", "fixed UI hint admission failed"));
    }
    let layers = generation_preview_layers(eval_json);
    if layers == "[]" {
        let scene = TextEditorScene::base(eval_json.to_string(), Some("json".into()), None);
        return crate::scene_surface(GENERATION2D_PLAY_SURFACE_GENERATE_PREVIEW, semio_framework_plugin::plugin_app_close_prelude::SurfaceKind::TextEditor, &scene);
    }
    let _ = GENERATION2D_PLAY_APP_ID;
    crate::scene_surface(
        GENERATION2D_PLAY_SURFACE_GENERATE_PREVIEW,
        semio_framework_plugin::plugin_app_close_prelude::SurfaceKind::Canvas2d,
        &Canvas2dScene { camera_x: config.camera.x, camera_y: config.camera.y, zoom: config.camera.zoom, layers_json: layers, snapshot: None },
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::generation2d::testkit::{app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn generate_preview_hints_without_evaluated_output() {
        let mut app = app().await;
        assert!(render_body(&mut app, GENERATION2D_PLAY_BODY_GENERATE_PREVIEW).await.contains("evaluate a generation"));
    }
}
//#endregion 🧪️Tests
