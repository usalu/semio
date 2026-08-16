//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::{HostMediaHandlerDeclaration, Plugin};

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) declares both owned artifacts (`gismap`,
/// `gisterrain`); `Gis2dPlayApp::app_schema()`/`Gis3dPlayApp::app_schema()` are registered
/// automatically by each `register_document_app` call below.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("gis")
        .label("GIS")
        .version("0.1.0")
        .artifact(crate::artifacts::gismap::declaration())
        .artifact(crate::artifacts::gisterrain::declaration())
        .host_media_handler(HostMediaHandlerDeclaration::two_d_svg_export(
            "s.gis.host-media.two-d-svg",
            crate::artifacts::gismap::artifact_kind(),
            crate::artifacts::gismap::GIS_MAP_SCHEMA,
            "gis2d",
            crate::artifacts::gismap::schema::gis2d_document_json_to_svg,
        )?)
        .document_app::<crate::apps::gis2d::Gis2dPlayApp>(crate::apps::gis2d::create_gis2d_app())
        .document_app::<crate::apps::gis3d::Gis3dPlayApp>(crate::apps::gis3d::create_gis3d_app())
        .try_build()
}
