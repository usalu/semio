
use super::*;

#[semio_framework_async_macros::async_test]
async fn create_en1997_viewer_builds_a_definition_for_this_dialect() {
    let def = create_en1997_viewer();
    assert_eq!(def.dialect, EN1997_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<En1997Viewer as ArtifactViewer>::DIALECT, EN1997_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_falls_back_to_a_text_node() {
    let snapshot = En1997Snapshot::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let json = semio_framework_plugin::testkit::project_and_retire_fixture_tree(<En1997Viewer as ArtifactViewer>::render("nope", &doc, &ConfigView { snapshot: &NoConfig::default() }).expect("viewer assembly"), &semio_framework_plugin::ViewModel::default()).expect("json");
    assert!(json.contains("Unknown body"));
}
