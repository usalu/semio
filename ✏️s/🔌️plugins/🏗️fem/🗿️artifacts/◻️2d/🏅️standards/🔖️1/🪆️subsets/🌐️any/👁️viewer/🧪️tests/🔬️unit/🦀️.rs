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
    let snapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotRetiring::new(<Fem2dViewer as ArtifactViewer>::reference_birth().expect("explicit reference snapshot producer")().expect("the reference snapshot births"));
    assert!(!snapshot.nodes.is_empty(), "expected the bundled example fixture's nodes");
}

#[test]
fn viewer_declares_no_mutation_action_anywhere() {
    let def = create_fem2d_viewer();
    let declared = def.actions.iter().chain(def.window_kinds.iter().flat_map(|kind| kind.actions.iter()));
    let editing: Vec<&str> = declared.filter(|action| action.kind == semio_plugin::ActionKind::Mutation).map(|action| action.id.as_str()).collect();
    assert!(editing.is_empty(), "a viewer is read-only; editing actions declared: {editing:?}");
}
