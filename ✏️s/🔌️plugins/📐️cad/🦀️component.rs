//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Self-registers `"3d.cad"`'s solid/mesh/dwg codecs — the compliant shape 🌀️procedural already
/// uses for `"3d.procedural"` (`register_dwg_mesh_bridge`, reached from that plugin root's
/// `.setup()`). Ticket 26/08/13/UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION D2 moved
/// these ten registrations here from `🎪️demonstrator`'s koordinator pane, which owned neither the
/// kind nor the codecs; none of the registrars involved is one of APA §6's covered set, so
/// `ArtifactDeclaration` has no field for them and this stays an imperative `.setup()` hook (the
/// same declaration gap 🌀️procedural's plugin root documents) rather than declarative data.
fn register_exports() {
    crate::artifacts::cad::io::register_host_io();
}

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1/W1b) replaces the old `.setup(engine::register)`
/// escape hatch for everything `ArtifactDeclaration` can express; `CadPlayApp::app_schema()` is
/// registered automatically by `register_document_app` below. `.setup()` carries only the host IO
/// registration `ArtifactDeclaration` still cannot model — see `register_exports`.
pub fn plugin() -> Plugin {
    Plugin::builder("cad")
        .label("CAD")
        .version("0.1.0")
        .setup(register_exports)
        .artifact(crate::artifacts::cad::declaration())
        .register_document_app::<crate::apps::cad::CadPlayApp>(crate::apps::cad::create_cad_app())
        .build()
}
