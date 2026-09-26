//! 🔺️ Playbook changes window — a real DiffView surface comparing the empty authored baseline with the live playbook.

use crate::PlaybookSnapshot;
use semio_framework_plugin::{LocalizedLabel, SurfaceKind, WindowKindDefinition, WindowOptions};
use semio_framework_ui_scene::DiffViewScene;

pub const PLAYBOOK_PLAY_WINDOW_CHANGES: &str = "playbook-changes";
pub const PLAYBOOK_PLAY_BODY_CHANGES: &str = "playbook.play.changes";
const PLAYBOOK_PLAY_SURFACE_CHANGES: &str = "playbook.play.changes";

pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: PLAYBOOK_PLAY_WINDOW_CHANGES.into(),
        label: LocalizedLabel::native("Changes", "Änderungen"),
        body_key: PLAYBOOK_PLAY_BODY_CHANGES.into(),
        surface_kind: SurfaceKind::DiffView,
        icon_id: "file-diff".into(),
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

pub fn scene(spec: &PlaybookSnapshot) -> DiffViewScene {
    DiffViewScene {
        before: protocol::json::to_json_string(&PlaybookSnapshot::default()),
        after: protocol::json::to_json_string(spec),
        language: Some("json".into()),
        mode: Some("unified".into()),
        domain_id: None,
    }
}

pub fn render(spec: &PlaybookSnapshot) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    semio_framework_plugin::scene_surface(PLAYBOOK_PLAY_SURFACE_CHANGES, semio_framework_ui_contract::SurfaceKind::DiffView, &scene(spec))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
