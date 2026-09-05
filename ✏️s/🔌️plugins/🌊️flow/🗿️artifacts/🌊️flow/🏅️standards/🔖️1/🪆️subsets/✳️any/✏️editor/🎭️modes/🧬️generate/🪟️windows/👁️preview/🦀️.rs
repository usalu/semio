//! 👁️ Generate-mode window — the evaluated output preview of the active generation.

use crate::editor::flow::config::FlowConfig;
use semio_framework_plugin::{scene_surface, BuiltNode, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};
use semio_framework_ui_contract::SurfaceKind as ContractSurfaceKind;
use ui_wgpu::wgpu::TextEditorScene;

//#region 🔖️Constants
pub const FLOW_PLAY_WINDOW_GENERATE_PREVIEW: &str = "flow-generate-preview";
pub const FLOW_PLAY_BODY_GENERATE_PREVIEW: &str = "flow.play.generate-preview";
const FLOW_PLAY_SURFACE_GENERATE_PREVIEW: &str = "flow.play.generate-preview";
/// 👁️ Shown until a generation has actually been evaluated — runtime status text, not authored UI copy.
const FLOW_PLAY_PREVIEW_PLACEHOLDER: &str = "(evaluate a generation to preview output)";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: FLOW_PLAY_WINDOW_GENERATE_PREVIEW.into(),
        label: LocalizedLabel::native("Preview", "Vorschau"),
        body_key: FLOW_PLAY_BODY_GENERATE_PREVIEW.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "eye".into(),
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
pub fn render(config: &FlowConfig) -> UiAssemblyResult<BuiltNode> {
    let generation = config.generation();
    let text = generation.preview_text.as_deref().filter(|value| !value.is_empty()).unwrap_or(FLOW_PLAY_PREVIEW_PLACEHOLDER);
    let scene = TextEditorScene::base(text.to_string(), Some("json".into()), None);
    scene_surface(FLOW_PLAY_SURFACE_GENERATE_PREVIEW, ContractSurfaceKind::TextEditor, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::flow::testkit::{flow_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn the_preview_renders_a_text_editor_surface() {
        let mut app = flow_app().await;
        assert!(render_body(&mut app, FLOW_PLAY_BODY_GENERATE_PREVIEW).await.contains("text-editor"));
    }
}
//#endregion 🧪️Tests
