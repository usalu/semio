use super::*;

#[test]
fn create_animate_presentation_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_animate_presentation_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, ANIMATE_DIALECT.into());
}

#[test]
fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<AnimatePresentationViewer as ArtifactViewer>::DIALECT, ANIMATE_DIALECT);
}
