//! 📋️ Playbook steps window — a real Table surface over the authored step sequence.

use crate::PlaybookSpec;
use semio_framework_value::ToValue;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::SurfaceKind;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_plugin::WindowOptions;
use semio_framework_ui_scene::TableScene;

pub const PLAYBOOK_PLAY_WINDOW_STEPS: &str = "playbook-steps";
pub const PLAYBOOK_PLAY_BODY_STEPS: &str = "playbook.play.steps";
const PLAYBOOK_PLAY_SURFACE_STEPS: &str = "playbook.play.steps";

pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: PLAYBOOK_PLAY_WINDOW_STEPS.into(),
        label: LocalizedLabel::native("Steps", "Schritte"),
        body_key: PLAYBOOK_PLAY_BODY_STEPS.into(),
        surface_kind: SurfaceKind::Table,
        icon_id: "table".into(),
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

pub fn scene(spec: &PlaybookSpec) -> TableScene {
    let columns = semio_framework_value::DslValue::Array(
        [("id", "Id"), ("title", "Title"), ("blocks", "Blocks")]
            .into_iter()
            .map(|(id, label)| semio_framework_value::DslValue::Object(vec![("id".into(), id.to_value()), ("label".into(), label.to_value()), ("sortable".into(), true.to_value())]))
            .collect(),
    );
    let rows = semio_framework_value::DslValue::Array(
        spec.steps
            .iter()
            .map(|step| semio_framework_value::DslValue::Object(vec![("id".into(), step.id.to_value()), ("title".into(), step.title.to_value()), ("blocks".into(), (step.blocks.len() as u64).to_value())]))
            .collect(),
    );
    TableScene::base(semio_framework_pack_json::to_json_string(&columns), semio_framework_pack_json::to_json_string(&rows))
}

pub fn render(spec: &PlaybookSpec) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    semio_framework_plugin::scene_surface(PLAYBOOK_PLAY_SURFACE_STEPS, semio_framework_ui_contract::SurfaceKind::Table, &scene(spec))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
