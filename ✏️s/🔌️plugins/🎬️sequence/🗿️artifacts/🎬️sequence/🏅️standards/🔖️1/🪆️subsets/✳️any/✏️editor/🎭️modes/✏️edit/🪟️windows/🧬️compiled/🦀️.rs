//! 🧬️ Sequence play app — the compiled-DAG window: the read-only wire literal of the current fixture.

use crate::artifacts::sequence::SequenceSnapshot;
use crate::editor::sequence::host_from_snapshot;
use semio_framework_plugin::{LocalizedLabel, SurfaceKind, TextEditorScene, BuiltNode, UiAssemblyResult, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const SEQUENCE_PLAY_WINDOW_COMPILED: &str = "sequence-compiled-dag";
pub const SEQUENCE_PLAY_BODY_COMPILED: &str = "sequence.play.compiled-dag";
const SEQUENCE_PLAY_SURFACE_COMPILED: &str = "sequence.play.compiled-dag";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: SEQUENCE_PLAY_WINDOW_COMPILED.into(),
        label: LocalizedLabel::native("DSL", "DSL"),
        body_key: SEQUENCE_PLAY_BODY_COMPILED.into(),
        surface_kind: SurfaceKind::NodeGraph,
        icon_id: "code".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(fixture: &SequenceSnapshot) -> UiAssemblyResult<BuiltNode> {
    let host = host_from_snapshot(fixture);
    semio_framework_plugin::scene_surface(SEQUENCE_PLAY_SURFACE_COMPILED, semio_framework_ui_contract::SurfaceKind::TextEditor, &TextEditorScene::base(host.compiled_wire_literal(), Some("wire".into()), None))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::sequence::testkit::{new_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn renders_compiled_wire_editor() {
        let mut app = new_app().await;
        assert!(render_body(&mut app, SEQUENCE_PLAY_BODY_COMPILED).await.contains("text-editor"));
    }
}
//#endregion 🧪️Tests
