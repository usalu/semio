use super::*;

#[semio_framework_async_macros::async_test]
async fn create_lowpoly_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_lowpoly_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, LOWPOLY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<LowpolyViewer as ArtifactViewer>::DIALECT, LOWPOLY_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn viewer_command_default_is_noop() {
    assert_eq!(LowpolyViewCommand::default(), LowpolyViewCommand::Noop);
}
