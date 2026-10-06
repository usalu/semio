//! 📜️ Playbook source window — a read-only TextEditor surface for the composed playbook (parent + `flow` child steps).

use crate::PlaybookSpec;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::SurfaceKind;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_plugin::WindowOptions;
use semio_framework_ui_scene::TextEditorScene;

pub const PLAYBOOK_PLAY_WINDOW_SOURCE: &str = "playbook-source";
pub const PLAYBOOK_PLAY_BODY_SOURCE: &str = "playbook.play.source";
const PLAYBOOK_PLAY_SURFACE_SOURCE: &str = "playbook.play.source";

pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
        id: PLAYBOOK_PLAY_WINDOW_SOURCE.into(),
        label: LocalizedLabel::native("Source", "Quelle"),
        body_key: PLAYBOOK_PLAY_BODY_SOURCE.into(),
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

pub fn scene(spec: &PlaybookSpec) -> TextEditorScene {
    TextEditorScene::base(semio_framework_pack_json::to_json_string(spec), Some("json".into()), None)
}

pub fn render(spec: &PlaybookSpec) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    semio_framework_plugin::scene_surface(PLAYBOOK_PLAY_SURFACE_SOURCE, semio_framework_ui_contract::SurfaceKind::TextEditor, &scene(spec))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
