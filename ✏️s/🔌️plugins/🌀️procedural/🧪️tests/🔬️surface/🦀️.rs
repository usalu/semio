
use crate::editor::generation2d::Generation2dPlayApp;
use crate::editor::generation3d::Generation3dPlayApp;
use crate::viewer::generation2d::Generation2dViewer;
use crate::viewer::generation3d::Generation3dViewer;

#[test]
fn plugin_manifest_builds_synchronously() {
    super::plugin().expect("procedural plugin manifest should build synchronously");
}

/// 👁️ A viewer instance never mutates the document store, even when dispatched.
#[semio_framework_async_macros::async_test]
async fn generation2d_viewer_never_mutates() {
    semio_framework_plugin::testkit::assert_viewer_never_mutates::<Generation2dViewer>().await;
}
#[semio_framework_async_macros::async_test]
async fn generation3d_viewer_never_mutates() {
    semio_framework_plugin::testkit::assert_viewer_never_mutates::<Generation3dViewer>().await;
}

/// 🤝️ Editor and viewer surfaces agree on the artifact dialect they address.
#[semio_framework_async_macros::async_test]
async fn generation2d_editor_and_viewer_share_dialect() {
    semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<Generation2dPlayApp, Generation2dViewer>().await;
}
#[semio_framework_async_macros::async_test]
async fn generation3d_editor_and_viewer_share_dialect() {
    semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<Generation3dPlayApp, Generation3dViewer>().await;
}

/// 📚️ Ticket 26/09/03/PROCEDURAL-3D-END-TO-END — `.editor_with_examples::<Generation3dPlayApp>`
/// must stamp the eight `crate::editor::generation3d::examples()` fixtures onto the manifest,
/// `app_id`-addressed to the gen3d editor surface, or the react shell's example dropdown
/// (`activePluginManifest.examples`) stays hidden for `generation3d`.
#[test]
fn generation3d_manifest_examples_are_registered_on_the_editor_surface() {
    let plugin = super::plugin().expect("procedural plugin manifest should build synchronously");
    let editor_app_id = crate::editor::generation3d::create_generation3d_app().id;
    assert_eq!(editor_app_id, "s.procedural.generation3d@1/*#editor");
    let registered_ids: Vec<&str> = plugin.manifest.examples.iter().filter(|example| example.app_id == editor_app_id).map(|example| example.id.as_str()).collect();
    let expected_sources = crate::editor::generation3d::examples();
    let expected_ids: Vec<&str> = expected_sources.iter().map(|source| source.id()).collect();
    assert_eq!(registered_ids.len(), 8);
    assert_eq!(registered_ids, expected_ids);
}
