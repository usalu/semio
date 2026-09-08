
use super::*;

#[semio_framework_async_macros::async_test]
async fn create_gismap_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_gismap_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, GISMAP_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<GisMapViewer as ArtifactViewer>::DIALECT, GISMAP_DIALECT);
}
