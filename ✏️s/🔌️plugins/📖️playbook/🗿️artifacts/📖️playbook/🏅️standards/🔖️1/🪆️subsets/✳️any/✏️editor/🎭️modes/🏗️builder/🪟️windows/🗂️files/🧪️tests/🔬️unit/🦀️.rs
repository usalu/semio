use super::*;
use crate::{schema::default_block, PlaybookStep};

#[test]
fn files_window_projects_the_fixture_hierarchy_without_navigation_or_selection_actions() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🎬️scene-showcase/🔣️.json")).expect("scene showcase fixture");
    let files = &fixture["files"];
    let mut block = default_block("project-name".into(), "text");
    block.label = "Project Name".into();
    let snapshot = crate::playbook_snapshot_with_steps(
        "playbook.program",
        "playbook",
        "1",
        Some("Playbook".into()),
        vec![
            PlaybookStep { id: "basics".into(), title: "Project Basics".into(), description: None, blocks: vec![block] },
            PlaybookStep { id: "publish".into(), title: "Publish".into(), description: None, blocks: Vec::new() },
        ],
    );
    let labels = crate::editor::playbook::terminology::playbook_play_labels(&semio_framework_plugin::ViewModel::default());
    let projected = scene(&snapshot, labels);
    let schema: serde_json::Value = serde_json::from_str(&projected.schema_json).expect("files schema");
    let rows: serde_json::Value = serde_json::from_str(&projected.rows_json).expect("files rows");
    assert_eq!(schema, files["schema"]);
    assert_eq!(rows, files["rows"]);
    assert_eq!(projected.selected_row_ids_json, None);
    assert_eq!(projected.drag_drop_enabled, Some(false));
    assert!(!projected.rows_json.contains("navigateUri"));
}
