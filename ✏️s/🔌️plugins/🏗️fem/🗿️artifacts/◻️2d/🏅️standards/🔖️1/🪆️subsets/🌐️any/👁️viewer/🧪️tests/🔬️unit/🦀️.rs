use super::*;

#[test]
fn create_fem2d_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_fem2d_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, FEM2D_DIALECT.into());
}

#[test]
fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Fem2dViewer as ArtifactViewer>::DIALECT, FEM2D_DIALECT);
}

#[test]
fn initial_snapshot_is_non_empty() {
    let snapshot = <Fem2dViewer as ArtifactViewer>::initial_snapshot();
    assert!(!snapshot.nodes.is_empty(), "expected the bundled example fixture's nodes");
}
