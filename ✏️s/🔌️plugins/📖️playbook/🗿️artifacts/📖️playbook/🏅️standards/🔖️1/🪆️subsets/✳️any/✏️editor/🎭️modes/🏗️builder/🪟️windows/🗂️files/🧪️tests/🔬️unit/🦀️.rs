use crate::PlaybookSpec;
use crate::editor::playbook::modes::builder::windows::files::*;
use crate::{schema::default_block, PlaybookStep};

#[test]
fn files_window_projects_the_fixture_hierarchy_without_navigation_or_selection_actions() {
    let fixture: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../../../✏️editor/🎭️modes/🏗️builder/🧪️tests/🧫️fixtures/🎬️scene-showcase/🔣️.json")).expect("scene showcase fixture");
    let files = &fixture["files"];
    let mut block = default_block("project-name".into(), "text");
    block.label = "Project Name".into();
    let snapshot = crate::PlaybookSpec {
        schema: "playbook.program".into(),
        id: "playbook".into(),
        version: "1".into(),
        title: Some("Playbook".into()),
        steps: vec![
            PlaybookStep { id: "basics".into(), title: "Project Basics".into(), description: None, blocks: vec![block] },
            PlaybookStep { id: "publish".into(), title: "Publish".into(), description: None, blocks: Vec::new() },
        ],
    };
    let labels = crate::editor::playbook::terminology::playbook_play_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native));
    let projected = scene(&snapshot, labels);
    let schema: serde_json::Value = serde_json::from_str(&projected.schema_json).expect("files schema");
    let rows: serde_json::Value = serde_json::from_str(&projected.rows_json).expect("files rows");
    assert_eq!(schema, files["schema"]);
    assert_eq!(rows, files["rows"]);
    assert_eq!(projected.selected_row_ids_json, None);
    assert_eq!(projected.drag_drop_enabled, Some(false));
    assert!(!projected.rows_json.contains("navigateUri"));
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
