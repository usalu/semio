
use super::*;

#[semio_framework_async_macros::async_test]
async fn create_en1999_viewer_builds_a_definition_for_this_dialect() {
    let def = create_en1999_viewer();
    assert_eq!(def.dialect, EN1999_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<En1999Viewer as ArtifactViewer>::DIALECT, EN1999_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_falls_back_to_a_text_node() {
    let snapshot = En1999Snapshot::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let json = semio_framework_plugin::testkit::project_and_retire_fixture_tree(<En1999Viewer as ArtifactViewer>::render("nope", &doc, &ConfigView { snapshot: &NoConfig::default() }).expect("viewer assembly")).expect("json");
    assert!(json.contains("Unknown body"));
}
