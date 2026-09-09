use super::*;

#[test]
fn create_playground_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_playground_viewer();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
    assert_eq!(def.dialect, PLAYGROUND_DIALECT.into());
}

#[test]
fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<PlaygroundViewer as ArtifactViewer>::DIALECT, PLAYGROUND_DIALECT);
}
