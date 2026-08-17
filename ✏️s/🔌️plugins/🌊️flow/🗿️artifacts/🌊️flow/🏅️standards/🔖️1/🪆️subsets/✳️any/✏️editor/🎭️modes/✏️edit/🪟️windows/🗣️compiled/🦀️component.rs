//! 🗣️ Flow play app — the compiled-DAG window: the read-only wire literal of the current fixture.

use crate::artifacts::flow::FlowSnapshot;
use crate::editor::flow::config::FlowConfig;
use crate::editor::flow::host_from_snapshot;
use crate::editor::flow::FLOW_PLAY_APP_ID;
use flow::FlowEvalSession;
use semio_framework_plugin::{build_text_editor_scene, LocalizedLabel, SurfaceKind, TextEditorScene, UiNode, WindowKindDefinition, WindowOptions};

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
pub fn render(fixture: &FlowSnapshot, config: &FlowConfig, session: &FlowEvalSession) -> UiNode {
    let host = host_from_snapshot(fixture, config, session);
    build_text_editor_scene(FLOW_PLAY_SURFACE_COMPILED, FLOW_PLAY_APP_ID, TextEditorScene::base(host.compiled_wire_literal(), Some("wire".into()), None))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::flow::testkit::{flow_app, render as render_body};

    #[test]
    fn renders_compiled_wire_editor() {
        let mut app = flow_app();
        assert!(render_body(&mut app, FLOW_PLAY_BODY_COMPILED).contains("text-editor"));
    }
}
//#endregion 🧪️Tests
