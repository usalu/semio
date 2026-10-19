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
    let mut scope = semio_framework_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = semio_framework_plugin::artifact_app_laws::new_viewer::<Generation2dViewer>(semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
    semio_framework_plugin::artifact_app_laws::render_registered_fixture_app(&mut app, preview::BODY_KEY, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native), scope.cx()).await.expect("the viewer renders and projects its default document");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, scope.cx()).expect("original caller paid app close");
}
