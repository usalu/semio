//! 📜️ Sequence play app — the script window: the compiled imperative path plus the last `run` result.

use super::transient::SequenceScriptWindowTransient;
use crate::editor::sequence::host_from_fixture;
use crate::SequenceFixture;
use semio_framework_plugin::{BuiltNode, LocalizedLabel, SurfaceKind, TextEditorScene, UiAssemblyResult, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const SEQUENCE_PLAY_WINDOW_SCRIPT: &str = "sequence-script";
pub const SEQUENCE_PLAY_BODY_SCRIPT: &str = "sequence.play.script";
const SEQUENCE_PLAY_SURFACE_SCRIPT: &str = "sequence.play.script";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: SEQUENCE_PLAY_WINDOW_SCRIPT.into(),
        label: LocalizedLabel::native("Script", "Skript"),
        body_key: SEQUENCE_PLAY_BODY_SCRIPT.into(),
        surface_kind: SurfaceKind::TextEditor,
        icon_id: "file-code".into(),
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
pub fn render(fixture: &SequenceFixture, transient: &SequenceScriptWindowTransient) -> UiAssemblyResult<BuiltNode> {
    let host = neural_engine::ColdOwner::new(host_from_fixture(fixture));
    let mut text = host.compile_text();
    if !transient.last_run_json.is_empty() {
        text.push_str("\n\n# run result\n");
        text.push_str(&transient.last_run_json);
    }
    semio_framework_plugin::scene_surface(SEQUENCE_PLAY_SURFACE_SCRIPT, semio_framework_ui_contract::SurfaceKind::TextEditor, &TextEditorScene::base(text, Some("imperative".into()), None))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
