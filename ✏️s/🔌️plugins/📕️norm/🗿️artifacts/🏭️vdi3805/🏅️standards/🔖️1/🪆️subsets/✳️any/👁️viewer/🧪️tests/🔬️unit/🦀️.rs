use super::*;

#[semio_framework_async_macros::async_test]
async fn create_vdi3805_viewer_builds_a_definition_for_this_dialect() {
    let def = create_vdi3805_viewer();
    assert_eq!(def.dialect, VDI3805_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Vdi3805Viewer as ArtifactViewer>::DIALECT, VDI3805_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_falls_back_to_a_text_node() {
    let snapshot = Vdi3805Snapshot::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(
        <Vdi3805Viewer as ArtifactViewer>::render("nope", &doc, &ConfigView { snapshot: &NoConfig::default(), window: None }, &semio_framework_plugin::ViewModel::default()).expect("viewer assembly"),
    )
    .expect("json");
    assert!(json.contains("Unknown body"));
}
