use super::*;

#[semio_framework_async_macros::async_test]
async fn create_writer_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_writer_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, WRITER_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<WriterViewer as ArtifactViewer>::DIALECT, WRITER_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    let document = schema::empty_writer_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let node = WriterViewer::render("writer.view.nope", &doc, &cfg, &semio_framework_plugin::ViewModel::default());
    assert!(semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(node.expect("viewer render")).unwrap().contains("Unknown body"));
}
