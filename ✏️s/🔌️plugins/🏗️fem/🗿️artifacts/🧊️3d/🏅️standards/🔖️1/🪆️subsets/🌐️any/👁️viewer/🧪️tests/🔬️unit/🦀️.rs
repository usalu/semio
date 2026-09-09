use super::*;

#[test]
fn create_fem3d_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_fem3d_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, crate::FEM3D_DIALECT.into());
}

#[test]
fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Fem3dViewer as ArtifactViewer>::DIALECT, crate::FEM3D_DIALECT);
}

#[test]
fn initial_snapshot_is_the_bundled_example_not_empty() {
    let snapshot = <Fem3dViewer as ArtifactViewer>::initial_snapshot();
    assert!(!snapshot.nodes.is_empty(), "expected the bundled default example's nodes");
}
