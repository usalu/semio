use semio_s_artifact_procedural_generation2d::editor::generation2d::Generation2dPlayApp;
use semio_s_artifact_procedural_generation2d::viewer::generation2d::Generation2dViewer;
use semio_s_artifact_procedural_generation3d::editor::generation3d::Generation3dPlayApp;
use semio_s_artifact_procedural_generation3d::viewer::generation3d::Generation3dViewer;


#[test]
fn plugin_manifest_builds_synchronously() {
    super::plugin().expect("procedural plugin manifest should build synchronously");
}

/// 👁️ A viewer instance never mutates the document store, even when dispatched.
#[semio_framework_async_macros::async_test]
async fn generation2d_viewer_never_mutates() {
    semio_framework_plugin::artifact_app_laws::assert_viewer_never_mutates::<Generation2dViewer>().await;
}
// 👁️ `Generation3dViewer` cannot use the same helper any more, and the law is not lost.
// `assert_viewer_never_mutates` is bounded `Presence = NoPresence, Transient = NoTransient`
// (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:7308`) because its `BoundedViewerFixture`
// falls back to `no_presence_store_disposer()`, which exists only for the zero-payload type — so the
// helper can only ever cover a viewer that owns NO state. The 3D viewer owns a real
// `Generation3dViewPresence` and `Generation3dViewTransient` since `📓️viewer-2026-09-09.md` §2.2-§2.3,
// which is exactly why the plugin's lib test target stopped compiling with four `E0271`s on this one
// line. The identical law now runs over the real viewer, with its real state, in
// `🗿️artifacts/🧊️generation3d/…/✳️any/👁️viewer/🧪️tests/🔬️unit/🦀️.rs`
// (`every_viewer_action_dispatches_live_and_never_mutates_the_document`), which dispatches all seven
// view commands through the interactive-job pipeline and asserts the document is untouched — strictly
// more than the one default command the helper sends.

/// 🤝️ Editor and viewer surfaces agree on the artifact dialect they address.
#[semio_framework_async_macros::async_test]
async fn generation2d_editor_and_viewer_share_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<Generation2dPlayApp, Generation2dViewer>().await;
}
#[semio_framework_async_macros::async_test]
async fn generation3d_editor_and_viewer_share_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<Generation3dPlayApp, Generation3dViewer>().await;
}

/// 📚️ Ticket 26/09/03/PROCEDURAL-3D-END-TO-END — `.editor_with_examples::<Generation3dPlayApp>`
/// must stamp the eight `semio_s_artifact_procedural_generation3d::editor::generation3d::examples()`
/// fixtures onto the manifest addressed to the gen3d DIALECT, or the react shell's example dropdown
/// (`activePluginManifest.examples`) stays hidden for `generation3d`.
///
/// 👁️ The same eight resolve for BOTH surfaces of that dialect: `manifest::examples_for_app` answers
/// the editor and the viewer identically, which is what gives the read-only surface its own picker
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn generation3d_manifest_examples_are_registered_on_the_dialect_for_both_surfaces() {
    let plugin = super::plugin().expect("procedural plugin manifest should build synchronously");
    let editor = semio_s_artifact_procedural_generation3d::editor::generation3d::create_generation3d_app();
    let viewer = semio_s_artifact_procedural_generation3d::viewer::generation3d::create_generation3d_viewer();
    assert_eq!(editor.id, "s.procedural.generation3d@1/*#editor");
    assert_eq!(viewer.id, "s.procedural.generation3d@1/*#viewer");
    assert_eq!(editor.dialect, viewer.dialect, "both surfaces are bound to one dialect");
    let expected_sources = semio_s_artifact_procedural_generation3d::editor::generation3d::examples();
    let expected_ids: Vec<&str> = expected_sources.iter().map(|source| source.id()).collect();
    for app in [&editor, &viewer] {
        let registered_ids: Vec<&str> = semio_framework_plugin::manifest::examples_for_app(&plugin.manifest.examples, app).into_iter().map(|example| example.id.as_str()).collect();
        assert_eq!(registered_ids.len(), 8, "{} must offer all eight bundled examples", app.id);
        assert_eq!(registered_ids, expected_ids, "{} must offer the authored example order", app.id);
    }
}

#[semio_framework_async_macros::async_test]
async fn assembly_editor_and_viewer_share_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<semio_s_artifact_procedural_assembly::editor::assembly::AssemblyEditor, semio_s_artifact_procedural_assembly::viewer::assembly::AssemblyViewer>().await;
}

#[test]
fn assembly_manifest_examples_are_registered_on_the_editor_surface() {
    let plugin = super::plugin().expect("procedural plugin manifest should build synchronously");
    let editor = semio_s_artifact_procedural_assembly::editor::assembly::create_assembly_editor();
    assert_eq!(editor.id, "s.assembly@1/*#editor");
    let registered_ids: Vec<&str> = semio_framework_plugin::manifest::examples_for_app(&plugin.manifest.examples, &editor).into_iter().map(|example| example.id.as_str()).collect();
    let sources = semio_s_artifact_procedural_assembly::examples::sources();
    let expected_ids: Vec<&str> = sources.iter().map(|source| source.id()).collect();
    assert_eq!(registered_ids, expected_ids);
    assert_eq!(registered_ids, ["two-room-corridor", "wall-roof-facade-strip"]);
}

#[test]
fn assembly_apps_are_declared_on_the_plugin() {
    let plugin = super::plugin().expect("procedural plugin manifest should build synchronously");
    let ids: Vec<&str> = plugin.manifest.apps.iter().map(|app| app.id.as_str()).collect();
    assert!(ids.iter().any(|id| *id == "s.assembly@1/*#editor"), "{ids:?}");
    assert!(ids.iter().any(|id| *id == "s.assembly@1/*#viewer"), "{ids:?}");
}
