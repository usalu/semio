//! 📜️ Sequence play app — the script window: the compiled imperative path plus the last `run` result.

use crate::apps::sequence::config::SequenceConfig;
use crate::apps::sequence::SEQUENCE_PLAY_APP_ID;
use crate::artifacts::sequence::engine::host_from_snapshot;
use crate::artifacts::sequence::SequenceSnapshot;
use semio_framework_plugin::{build_text_editor_scene, LocalizedLabel, SurfaceKind, TextEditorScene, UiNode, WindowKindDefinition, WindowOptions};

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
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(fixture: &SequenceSnapshot, config: &SequenceConfig) -> UiNode {
    let host = host_from_snapshot(fixture);
    let mut text = host.compile_text();
    if !config.last_run_json.is_empty() {
        text.push_str("\n\n# run result\n");
        text.push_str(&config.last_run_json);
    }
    build_text_editor_scene(SEQUENCE_PLAY_SURFACE_SCRIPT, SEQUENCE_PLAY_APP_ID, TextEditorScene::base(text, Some("imperative".into()), None))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::sequence::testkit::{new_app, render as render_body};

    #[test]
    fn renders_script_editor() {
        let mut app = new_app();
        assert!(render_body(&mut app, SEQUENCE_PLAY_BODY_SCRIPT).contains("text-editor"));
    }

    #[test]
    fn definition_declares_the_text_editor_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, SEQUENCE_PLAY_BODY_SCRIPT);
        assert!(matches!(definition.surface_kind, SurfaceKind::TextEditor));
    }
}
//#endregion 🧪️Tests
