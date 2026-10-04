//! 🗂️ Playbook files window — a deterministic read-only hierarchy over current steps and blocks.

use crate::editor::playbook::terminology::PlaybookPlayLabels;
use crate::PlaybookSpec;
use semio_framework_value::ToValue;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::SurfaceKind;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_plugin::WindowOptions;
use semio_framework_ui_scene::VirtualFileSystemScene;

pub const PLAYBOOK_PLAY_WINDOW_FILES: &str = "playbook-files";
pub const PLAYBOOK_PLAY_BODY_FILES: &str = "playbook.play.files";
const PLAYBOOK_PLAY_SURFACE_FILES: &str = "playbook.play.files";
const PLAYBOOK_PLAY_FILES_ROOT: &str = "playbook";

pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: PLAYBOOK_PLAY_WINDOW_FILES.into(),
        label: LocalizedLabel::native("Files", "Dateien"),
        body_key: PLAYBOOK_PLAY_BODY_FILES.into(),
        surface_kind: SurfaceKind::VirtualFileSystem,
        icon_id: "folder".into(),
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

fn file_node_kind(id: &str, name: &str, icon: &str) -> semio_framework_value::DslValue {
    semio_framework_value::DslValue::Object(vec![("id".into(), id.to_value()), ("name".into(), name.to_value()), ("icon".into(), icon.to_value()), ("descriptors".into(), semio_framework_value::DslValue::Array(Vec::new()))])
}

fn row(id: String, file_node_kind_id: &str, name: String, parent_id: Option<String>, has_children: bool) -> semio_framework_value::DslValue {
    semio_framework_value::DslValue::Object(vec![
        ("id".into(), id.to_value()),
        ("fileNodeKindId".into(), file_node_kind_id.to_value()),
        ("name".into(), name.to_value()),
        ("parentId".into(), parent_id.to_value()),
        ("hasChildren".into(), has_children.to_value()),
        ("descriptorValues".into(), semio_framework_value::DslValue::Object(Vec::new())),
    ])
}

pub fn scene(spec: &PlaybookSpec, labels: &PlaybookPlayLabels) -> VirtualFileSystemScene {
    let steps = &spec.steps;
    let schema = semio_framework_value::DslValue::Object(vec![
        (
            "fileNodeKinds".into(),
            semio_framework_value::DslValue::Object(vec![
                ("root".into(), file_node_kind("root", labels.files_root.as_str(), "layout-grid")),
                ("step".into(), file_node_kind("step", labels.files_step.as_str(), "folder")),
                ("block".into(), file_node_kind("block", labels.files_block.as_str(), "file")),
            ]),
        ),
        ("descriptorKinds".into(), semio_framework_value::DslValue::Object(Vec::new())),
        ("descriptorColumnIds".into(), semio_framework_value::DslValue::Array(Vec::new())),
    ]);
    let mut rows = Vec::new();
    rows.push(row(
        PLAYBOOK_PLAY_FILES_ROOT.into(),
        "root",
        spec.title.clone().filter(|title| !title.is_empty()).unwrap_or_else(|| labels.files_root.as_str().to_string()),
        None,
        !steps.is_empty(),
    ));
    for step in steps {
        let step_id = format!("step/{}", step.id);
        let step_name = if step.title.is_empty() { step.id.clone() } else { step.title.clone() };
        rows.push(row(step_id.clone(), "step", step_name, Some(PLAYBOOK_PLAY_FILES_ROOT.into()), !step.blocks.is_empty()));
        for block in &step.blocks {
            let block_name = if block.label.is_empty() { block.id.clone() } else { block.label.clone() };
            rows.push(row(format!("block/{}/{}", step.id, block.id), "block", block_name, Some(step_id.clone()), false));
        }
    }
    VirtualFileSystemScene {
        schema_json: semio_framework_pack_json::to_json_string(&schema),
        rows_json: semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::Array(rows)),
        selected_row_ids_json: None,
        hovered_row_id: None,
        empty_message: None,
        drag_drop_enabled: Some(false),
    }
}

pub fn render(spec: &PlaybookSpec, labels: &PlaybookPlayLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    semio_framework_plugin::scene_surface(PLAYBOOK_PLAY_SURFACE_FILES, semio_framework_ui_contract::SurfaceKind::VirtualFileSystem, &scene(spec, labels))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
