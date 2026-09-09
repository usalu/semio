use super::*;

#[test]
fn create_generation2d_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_generation2d_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, GENERATION2D_DIALECT.into());
}

#[test]
fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Generation2dViewer as ArtifactViewer>::DIALECT, GENERATION2D_DIALECT);
}
