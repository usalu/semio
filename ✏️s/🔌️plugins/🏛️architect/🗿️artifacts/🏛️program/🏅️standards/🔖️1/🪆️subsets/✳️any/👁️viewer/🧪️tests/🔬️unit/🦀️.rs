use super::*;

#[semio_framework_async_macros::async_test]
async fn create_architect_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_architect_viewer();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
    assert_eq!(def.dialect, ARCHITECT_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<ArchitectViewer as ArtifactViewer>::DIALECT, ARCHITECT_DIALECT);
}
