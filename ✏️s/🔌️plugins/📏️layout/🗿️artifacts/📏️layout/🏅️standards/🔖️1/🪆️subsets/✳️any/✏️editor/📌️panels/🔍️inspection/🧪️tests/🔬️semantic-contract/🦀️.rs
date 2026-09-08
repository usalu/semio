
use super::*;
#[test]
fn layout_inspection_summary_matches_the_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🔣️summary.json")).expect("neutral inspector vectors");
    let mut snapshot = crate::schema::default_document();
    snapshot.name = fixture["name"].as_str().expect("document name").into();
    snapshot.pages.clear();
    assert_eq!(snapshot.pages.len(), fixture["pageCount"].as_u64().expect("page count") as usize);
    for row in fixture["cases"].as_array().expect("locales") {
        let config = LayoutConfig { locale: row["locale"].as_str().expect("locale").into(), active_page_id: fixture["activePage"].as_str().expect("active page").into(), ..LayoutConfig::default() };
        let node = render(&snapshot, &config, crate::editor::layout::terminology::layout_labels(&config)).expect("semantic inspector");
        let projection = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project and retire inspector");
        let actual: serde_json::Value = serde_json::from_str(&projection).expect("independent semantic JSON oracle");
        assert_eq!(actual["component"]["label"], row["heading"]);
        let lines: Vec<_> = actual["children"].as_array().expect("summary lines").iter().map(|child| child["component"]["value"].clone()).collect();
        assert_eq!(lines, *row["lines"].as_array().expect("expected lines"));
    }
}
