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

/// ⚖️ LAW (S15 viewer matrix, 2026-09-25): opening the viewer panicked in the guest with `ordered-map root must be
/// explicitly retired before drop` — the viewer retired its documents through the framework's generic bounded owners,
/// which drop the snapshot's `OrderedMap` plainly, while the artifact declares its own retirement owners. The viewer
/// opens, reads and closes its document through the artifact's owners.
#[semio_framework_async_macros::async_test]
async fn the_viewer_opens_and_closes_its_document_through_the_artifacts_owners() {
    let mut app = semio_framework_plugin::artifact_app_laws::new_viewer::<Generation2dViewer>().await;
    let tree = semio_framework_plugin::PluginApp::render(&mut app, preview::BODY_KEY, None, &semio_framework_plugin::ViewModel::default()).await.expect("the viewer renders its default document");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("the preview projects");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
