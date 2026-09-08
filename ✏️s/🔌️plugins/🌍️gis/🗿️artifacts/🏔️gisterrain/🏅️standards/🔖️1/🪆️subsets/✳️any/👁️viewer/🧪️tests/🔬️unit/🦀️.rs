
use super::*;

#[semio_framework_async_macros::async_test]
async fn create_gisterrain_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_gisterrain_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, GISTERRAIN_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<GisTerrainViewer as ArtifactViewer>::DIALECT, GISTERRAIN_DIALECT);
}
