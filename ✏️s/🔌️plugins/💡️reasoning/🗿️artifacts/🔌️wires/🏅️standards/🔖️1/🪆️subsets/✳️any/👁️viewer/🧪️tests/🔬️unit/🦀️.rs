
use super::*;

#[semio_framework_async_macros::async_test]
async fn create_wires_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_wires_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, WIRES_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<WiresViewer as ArtifactViewer>::DIALECT, WIRES_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn viewer_command_default_is_noop() {
    assert_eq!(WiresViewCommand::default(), WiresViewCommand::Noop);
}
