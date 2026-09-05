//! 🗣️ Flow play app — the compiled-DAG window: the read-only wire literal of the current fixture.

use crate::artifacts::flow::FlowSnapshot;
use crate::editor::flow::config::FlowConfig;
use crate::editor::flow::host_from_snapshot;
use flow::FlowEvalSession;
use semio_framework_plugin::{scene_surface, BuiltNode, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};
use semio_framework_ui_contract::SurfaceKind as ContractSurfaceKind;
use ui_wgpu::wgpu::TextEditorScene;

//#region 🔖️Constants
pub const FLOW_PLAY_WINDOW_COMPILED: &str = "flow-compiled-dag";
pub const FLOW_PLAY_BODY_COMPILED: &str = "flow.play.compiled-dag";
const FLOW_PLAY_SURFACE_COMPILED: &str = "flow.play.compiled-dag";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: FLOW_PLAY_WINDOW_COMPILED.into(),
        label: LocalizedLabel::native("DSL", "DSL"),
        body_key: FLOW_PLAY_BODY_COMPILED.into(),
        surface_kind: SurfaceKind::NodeGraph,
        icon_id: "code".into(),
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
pub fn render(fixture: &FlowSnapshot, config: &FlowConfig, session: &FlowEvalSession) -> UiAssemblyResult<BuiltNode> {
    let host = host_from_snapshot(fixture, config, session);
    let scene = TextEditorScene::base(host.compiled_wire_literal(), Some("wire".into()), None);
    scene_surface(FLOW_PLAY_SURFACE_COMPILED, ContractSurfaceKind::TextEditor, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::flow::testkit::{flow_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn renders_compiled_wire_editor() {
        let mut app = flow_app().await;
        assert!(render_body(&mut app, FLOW_PLAY_BODY_COMPILED).await.contains("text-editor"));
    }
}
//#endregion 🧪️Tests
