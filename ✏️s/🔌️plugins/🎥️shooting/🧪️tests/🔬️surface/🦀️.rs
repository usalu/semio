//! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5 promises
//! `semio_framework_plugin::artifact_app_laws::{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect,
//! new_viewer}` — landed for real as of this packet (unlike the cad pilot, which had to write local
//! stand-ins), so these call the canonical framework versions directly.
use semio_framework_plugin::artifact_app_laws::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn shooting_viewer_never_mutates() {
    assert_viewer_never_mutates::<crate::viewer::shooting::ShootingViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn shooting_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<crate::editor::shooting::ShootingPlayApp, crate::viewer::shooting::ShootingViewer>().await;
}

#[test]
fn shooting_manifest_examples_are_registered_for_the_editor_surface() {
    let plugin = super::plugin().expect("shooting plugin manifest should build");
    let editor = crate::editor::shooting::create_shooting_app();
    let viewer = crate::viewer::shooting::create_shooting_viewer();
    assert_eq!(editor.dialect, viewer.dialect);
    let expected = vec![
        semio_s_artifact_shooting_shooting::examples::demo::source(),
        semio_s_artifact_shooting_shooting::examples::hexagonal_cut_concrete_forest_left::source(),
    ];
    let expected_ids: Vec<&str> = expected.iter().map(|source| source.id()).collect();
    for app in [&editor, &viewer] {
        let registered_ids: Vec<&str> = semio_framework_plugin::manifest::examples_for_app(&plugin.manifest.examples, app).into_iter().map(|example| example.id.as_str()).collect();
        assert_eq!(registered_ids, expected_ids, "{} must expose the bundled examples", app.id);
    }
}
