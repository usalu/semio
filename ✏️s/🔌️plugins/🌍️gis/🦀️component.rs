//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::{HostMediaHandlerDeclaration, Plugin};

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) declares both owned artifacts (`gismap`,
/// `gisterrain`); `Gis2dPlayApp::app_schema()`/`Gis3dPlayApp::app_schema()` are registered
/// automatically by each `.editor()` call below. `.editor()`/`.viewer()` (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET) replace the retired `.document_app()` — each
/// subset now registers an independent editor and viewer surface.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("gis")
        .label("GIS")
        .version("0.1.0")
        .artifact(crate::artifacts::gismap::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .artifact(crate::artifacts::gisterrain::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .host_media_handler(HostMediaHandlerDeclaration::two_d_svg_export(
            "s.gis.host-media.two-d-svg",
            crate::artifacts::gismap::artifact_kind(),
            crate::artifacts::gismap::GIS_MAP_SCHEMA,
            "gis2d",
            crate::artifacts::gismap::schema::gis2d_document_json_to_svg,
        )?)
        .editor::<crate::editor::gis2d::Gis2dPlayApp>(crate::editor::gis2d::create_gis2d_app())
        .editor_mutation_roster::<crate::editor::gis2d::Gis2dPlayApp>()
        .viewer::<crate::viewer::gismap::GisMapViewer>(crate::viewer::gismap::create_gismap_viewer())
        .viewer_mutation_roster::<crate::viewer::gismap::GisMapViewer>()
        .editor::<crate::editor::gis3d::Gis3dPlayApp>(crate::editor::gis3d::create_gis3d_app())
        .editor_mutation_roster::<crate::editor::gis3d::Gis3dPlayApp>()
        .viewer::<crate::viewer::gisterrain::GisTerrainViewer>(crate::viewer::gisterrain::create_gisterrain_viewer())
        .viewer_mutation_roster::<crate::viewer::gisterrain::GisTerrainViewer>()
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
mod surface_tests {
    //! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5 — the real
    //! `semio_framework_plugin::testkit::{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect,
    //! new_viewer}` (closed by w0-f, gap 2), used directly rather than local stand-ins.
    use crate::editor::gis2d::Gis2dPlayApp;
    use crate::editor::gis3d::Gis3dPlayApp;
    use crate::viewer::gismap::GisMapViewer;
    use crate::viewer::gisterrain::GisTerrainViewer;
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[test]
    fn gismap_viewer_never_mutates() {
        assert_viewer_never_mutates::<GisMapViewer>();
    }

    #[test]
    fn gismap_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<Gis2dPlayApp, GisMapViewer>();
    }

    #[test]
    fn gisterrain_viewer_never_mutates() {
        assert_viewer_never_mutates::<GisTerrainViewer>();
    }

    #[test]
    fn gisterrain_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<Gis3dPlayApp, GisTerrainViewer>();
    }
}
//#endregion 🧪️SurfaceTests
