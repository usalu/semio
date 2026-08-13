//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Self-registers `"2d.map"`'s svg/png/dwg export + dwg import handlers — the compliant shape
/// 🌀️procedural already uses for `"3d.procedural"` (`register_dwg_mesh_bridge`, reached from that
/// plugin root's `.setup()`). Ticket 26/08/13/UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION
/// D2 moved these two registrations here from `🎪️demonstrator`'s verfolgen pane, which owned neither
/// the kind nor the codecs; neither registrar is in APA §6's covered set, so `ArtifactDeclaration`
/// has no field for them and this stays an imperative `.setup()` hook (the same declaration gap
/// 🌀️procedural's plugin root documents) rather than declarative data.
fn register_exports() {
    crate::artifacts::gismap::io::register_host_io();
}

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old `.setup(engine::register)`
/// escape hatch for everything `ArtifactDeclaration` can express on both owned artifacts (`gismap`,
/// `gisterrain`); `Gis2dPlayApp::app_schema()`/`Gis3dPlayApp::app_schema()` are registered
/// automatically by each `register_document_app` call below. `.setup()` carries only the host IO
/// registration `ArtifactDeclaration` still cannot model — see `register_exports`.
pub fn plugin() -> Plugin {
    Plugin::builder("gis")
        .label("GIS")
        .version("0.1.0")
        .setup(register_exports)
        .artifact(crate::artifacts::gismap::declaration())
        .artifact(crate::artifacts::gisterrain::declaration())
        .register_document_app::<crate::apps::gis2d::Gis2dPlayApp>(crate::apps::gis2d::create_gis2d_app())
        .register_document_app::<crate::apps::gis3d::Gis3dPlayApp>(crate::apps::gis3d::create_gis3d_app())
        .build()
}
