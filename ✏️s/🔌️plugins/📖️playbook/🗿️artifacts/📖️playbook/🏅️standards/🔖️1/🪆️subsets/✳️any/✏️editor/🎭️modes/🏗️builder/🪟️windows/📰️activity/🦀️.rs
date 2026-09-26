//! 📰️ Playbook activity window — a deterministic EventFeed projection of the authored step sequence.

use crate::PlaybookSnapshot;
use dsl::ToValue;
use semio_framework_plugin::{LocalizedLabel, SurfaceKind, WindowKindDefinition, WindowOptions};
use semio_framework_ui_scene::EventFeedScene;

pub const PLAYBOOK_PLAY_WINDOW_ACTIVITY: &str = "playbook-activity";
pub const PLAYBOOK_PLAY_BODY_ACTIVITY: &str = "playbook.play.activity";
const PLAYBOOK_PLAY_SURFACE_ACTIVITY: &str = "playbook.play.activity";

pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: PLAYBOOK_PLAY_WINDOW_ACTIVITY.into(),
        label: LocalizedLabel::native("Activity", "Aktivität"),
        body_key: PLAYBOOK_PLAY_BODY_ACTIVITY.into(),
        surface_kind: SurfaceKind::EventFeed,
        icon_id: "list-checks".into(),
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

pub fn scene(spec: &PlaybookSnapshot) -> EventFeedScene {
    let entries = dsl::DslValue::Array(
        spec.steps()
            .iter()
            .enumerate()
            .map(|(index, step)| {
                dsl::DslValue::Object(vec![
                    ("id".into(), step.id.to_value()),
                    ("timestampMs".into(), (index as u64).to_value()),
                    ("iconId".into(), "list-checks".to_value()),
                    ("title".into(), step.title.to_value()),
                    ("detail".into(), format!("{} blocks", step.blocks.len()).to_value()),
                    ("tone".into(), "info".to_value()),
                ])
            })
            .collect(),
    );
    EventFeedScene { entries_json: protocol::json::to_json_string(&entries), follow: Some(false), activate_action: None, domain_id: None }
}

pub fn render(spec: &PlaybookSnapshot) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    semio_framework_plugin::scene_surface(PLAYBOOK_PLAY_SURFACE_ACTIVITY, semio_framework_ui_contract::SurfaceKind::EventFeed, &scene(spec))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
