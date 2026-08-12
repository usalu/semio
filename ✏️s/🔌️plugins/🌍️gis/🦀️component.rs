//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old `.setup(engine::register)`
/// escape hatch for both owned artifacts (`gismap`, `gisterrain`); `.setup()` itself is gone (W1c)
/// — `Gis2dPlayApp::app_schema()`/`Gis3dPlayApp::app_schema()` now answer the one thing it used to
/// survive for, registered automatically by each `register_document_app` call below.
pub fn plugin() -> Plugin {
    Plugin::builder("gis")
        .label("GIS")
        .version("0.1.0")
        .artifact(crate::artifacts::gismap::declaration())
        .artifact(crate::artifacts::gisterrain::declaration())
        .register_document_app::<crate::apps::gis2d::Gis2dPlayApp>(crate::apps::gis2d::create_gis2d_app())
        .register_document_app::<crate::apps::gis3d::Gis3dPlayApp>(crate::apps::gis3d::create_gis3d_app())
        .build()
}
