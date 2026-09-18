use semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect;

/// 🔌️ The manifest is assembled synchronously — five artifacts, five routed inferences, ten apps.
#[test]
fn plugin_manifest_builds_synchronously() {
    super::plugin().expect("wfc plugin manifest should build synchronously");
}

/// 🤝️ Editor and viewer surfaces of one artifact address the same dialect. A surface pair that
/// drifts apart is a viewer that silently opens nothing for the documents its editor writes.
macro_rules! wfc_dialect_law {
    ($name:ident, $editor:ty, $viewer:ty) => {
        #[semio_framework_async_macros::async_test]
        async fn $name() {
            assert_editor_and_viewer_share_dialect::<$editor, $viewer>().await;
        }
    };
}

wfc_dialect_law!(bitmap_editor_and_viewer_share_dialect, semio_s_artifact_wfc_bitmap::editor::bitmap::BitmapEditor, semio_s_artifact_wfc_bitmap::viewer::bitmap::BitmapViewer);
wfc_dialect_law!(grid2d_editor_and_viewer_share_dialect, semio_s_artifact_wfc_grid2d::editor::grid2d::Grid2dEditor, semio_s_artifact_wfc_grid2d::viewer::grid2d::Grid2dViewer);
wfc_dialect_law!(wfc2d_editor_and_viewer_share_dialect, semio_s_artifact_wfc_2d::editor::wfc2d::Wfc2dEditor, semio_s_artifact_wfc_2d::viewer::wfc2d::Wfc2dViewer);
wfc_dialect_law!(grid3d_editor_and_viewer_share_dialect, semio_s_artifact_wfc_grid3d::editor::grid3d::Grid3dEditor, semio_s_artifact_wfc_grid3d::viewer::grid3d::Grid3dViewer);
wfc_dialect_law!(wfc3d_editor_and_viewer_share_dialect, semio_s_artifact_wfc_3d::editor::wfc3d::Wfc3dEditor, semio_s_artifact_wfc_3d::viewer::wfc3d::Wfc3dViewer);

/// 🗃️ LAW: all ten surfaces reach the manifest. `.declare_artifact(…)` is the only registration
/// channel, so one artifact left out of the builder is an editor the host can never activate.
#[test]
fn every_wfc_surface_is_declared_on_the_plugin() {
    let plugin = super::plugin().expect("wfc plugin manifest should build synchronously");
    let ids: Vec<String> = plugin.manifest.apps.iter().map(|app| app.id.to_string()).collect();
    for dialect in ["s.wfc.bitmap", "s.wfc.grid2d", "s.wfc.wfc2d", "s.wfc.grid3d", "s.wfc.wfc3d"] {
        for surface in ["editor", "viewer"] {
            let expected = format!("{dialect}@1/*#{surface}");
            assert!(ids.iter().any(|id| *id == expected), "{expected} must be declared — got {ids:?}");
        }
    }
}

/// 📚️ LAW: each artifact's authored examples are stamped onto its dialect for BOTH surfaces, or the
/// react shell's example dropdown (`activePluginManifest.examples`) stays empty for that variant.
macro_rules! wfc_examples_law {
    ($name:ident, $editor:expr, $viewer:expr, $sources:expr) => {
        #[test]
        fn $name() {
            let plugin = super::plugin().expect("wfc plugin manifest should build synchronously");
            let editor = $editor;
            let viewer = $viewer;
            assert_eq!(editor.dialect, viewer.dialect, "both surfaces are bound to one dialect");
            let expected_ids: Vec<&str> = $sources.iter().map(|source| source.id()).collect();
            assert!(expected_ids.len() >= 2, "every wfc artifact ships at least a tiny and a real-world example");
            for app in [&editor, &viewer] {
                let registered_ids: Vec<&str> = semio_framework_plugin::manifest::examples_for_app(&plugin.manifest.examples, app).into_iter().map(|example| example.id.as_str()).collect();
                assert_eq!(registered_ids, expected_ids, "{} must offer the authored example order", app.id);
            }
        }
    };
}

wfc_examples_law!(bitmap_examples_are_registered_for_both_surfaces, semio_s_artifact_wfc_bitmap::editor::bitmap::create_bitmap_editor(), semio_s_artifact_wfc_bitmap::viewer::bitmap::create_bitmap_viewer(), semio_s_artifact_wfc_bitmap::examples::example_source_slice());
wfc_examples_law!(grid2d_examples_are_registered_for_both_surfaces, semio_s_artifact_wfc_grid2d::editor::grid2d::create_grid2d_editor(), semio_s_artifact_wfc_grid2d::viewer::grid2d::create_grid2d_viewer(), semio_s_artifact_wfc_grid2d::examples::grid2d::example_source_slice());
wfc_examples_law!(wfc2d_examples_are_registered_for_both_surfaces, semio_s_artifact_wfc_2d::editor::wfc2d::create_wfc2d_editor(), semio_s_artifact_wfc_2d::viewer::wfc2d::create_wfc2d_viewer(), semio_s_artifact_wfc_2d::examples::example_source_slice());
wfc_examples_law!(grid3d_examples_are_registered_for_both_surfaces, semio_s_artifact_wfc_grid3d::editor::grid3d::create_grid3d_editor(), semio_s_artifact_wfc_grid3d::viewer::grid3d::create_grid3d_viewer(), semio_s_artifact_wfc_grid3d::examples::example_source_slice());
wfc_examples_law!(wfc3d_examples_are_registered_for_both_surfaces, semio_s_artifact_wfc_3d::editor::wfc3d::create_wfc3d_editor(), semio_s_artifact_wfc_3d::viewer::wfc3d::create_wfc3d_viewer(), semio_s_artifact_wfc_3d::examples::example_source_slice());

/// 💡️ LAW: every artifact's routed inference metadata names THIS owner and its own DIALECT kind.
/// `ArtifactInferenceServiceRegistry` keys a service by `(artifact_kind, inference_schema)` and the
/// caller addresses it with the document's dialect `artifact_kind` (`s.wfc.<ident>`), not with the
/// OS `artifact_kind().id` the activation uses (`2d.wfc…`). A row that drifts from the dialect is a
/// solve tool the host lists but can never address — the same pairing `📐️cad` and `🧩️puzzle` ship.
#[test]
fn every_wfc_inference_route_names_its_own_artifact_kind() {
    let routes = [
        (semio_s_artifact_wfc_bitmap::standards::v1::subsets::any::schema::inferences::bitmap_inference_metadata(), semio_s_artifact_wfc_bitmap::WFC_BITMAP_DOCUMENT_SCHEMA),
        (semio_s_artifact_wfc_grid2d::standards::v1::subsets::any::schema::inferences::grid2d_inference_metadata(), semio_s_artifact_wfc_grid2d::WFC_GRID2D_DOCUMENT_SCHEMA),
        (semio_s_artifact_wfc_2d::standards::v1::subsets::any::schema::inferences::wfc2d_inference_metadata(), semio_s_artifact_wfc_2d::WFC_2D_DOCUMENT_SCHEMA),
        (semio_s_artifact_wfc_grid3d::standards::v1::subsets::any::schema::inferences::grid3d_inference_metadata(), semio_s_artifact_wfc_grid3d::WFC_GRID3D_DOCUMENT_SCHEMA),
        (semio_s_artifact_wfc_3d::standards::v1::subsets::any::schema::inferences::wfc3d_inference_metadata(), semio_s_artifact_wfc_3d::WFC3D_DOCUMENT_SCHEMA),
    ];
    for (metadata, dialect_kind) in routes {
        assert_eq!(metadata.owner, "wfc", "{} must be owned by the wfc plugin", metadata.inference_schema);
        assert_eq!(metadata.artifact_kind, dialect_kind, "{} must address its own dialect kind", metadata.inference_schema);
    }
}
