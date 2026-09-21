use super::*;
#[test]
fn layout_inspection_summary_matches_the_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️summary.json")).expect("neutral inspector vectors");
    let mut snapshot = crate::standards::v1::subsets::any::schema::default_document();
    snapshot.name = fixture["name"].as_str().expect("document name").into();
    snapshot.pages.clear();
    assert_eq!(snapshot.pages.len(), fixture["pageCount"].as_u64().expect("page count") as usize);
    for row in fixture["cases"].as_array().expect("locales") {
        let config = LayoutWindowConfig { active_page_id: fixture["activePage"].as_str().expect("active page").into(), ..LayoutWindowConfig::default() };
        let view_state = semio_framework_plugin::ViewModel { locale: semio_framework_plugin::locale_from_str(row["locale"].as_str().expect("locale")), ..Default::default() };
        let node = render(&snapshot, &config, &crate::editor::layout::LayoutInteractionSnapshot::default(), crate::editor::layout::terminology::layout_labels(&view_state)).expect("semantic inspector");
        let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project and retire inspector");
        let actual: serde_json::Value = serde_json::from_str(&projection).expect("independent semantic JSON oracle");
        // 🌳️ The inspector root is the panel TREE; its heading and rows live on the one summary
        // SECTION below it (`Component::TreeSection`/`TreeItem`, `label` + `description`), never on
        // the root and never as a single `value` string.
        let section = &actual["children"][0];
        assert_eq!(section["component"]["label"], row["heading"]);
        let lines: Vec<_> = section["children"]
            .as_array()
            .expect("summary lines")
            .iter()
            .map(|child| {
                let label = child["component"]["label"].as_str().expect("summary row label");
                let description = child["component"]["description"].as_str().expect("summary row description");
                serde_json::Value::String(format!("{label}: {description}"))
            })
            .collect();
        assert_eq!(lines, *row["lines"].as_array().expect("expected lines"));
    }
}
