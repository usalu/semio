//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Everything left in `.setup()` after the ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1
/// conversion — `.artifact(…)` now carries procedural2d's and procedural3d's own
/// schema/inference/composer/language/document-codec registrations as data. Four calls remain here
/// because `ArtifactDeclaration` has no field for any of them (all four named, not silently
/// dropped — see `📓️w1b-semio-s-plugin-procedural-report.md`):
/// - the two `register_app_schema()` calls are app-scope config/presence schema, the one §6
///   registrar (`register_app_schema_descriptor`) `ArtifactDeclaration` deliberately excludes
///   (same exception note's exemplar documents);
/// - `register_dwg_mesh_bridge()` (self-registering procedural3d's OWN kind — the compliant shape,
///   unlike a foreign plugin naming a kind it doesn't own) has no equivalent field because
///   `register_mesh_dwg_import_handler` isn't one of the §6 registrars `ArtifactDeclaration` was
///   built to cover — a genuine declaration gap, reported prominently rather than silently kept;
/// - `ensure_linked_flow_extensions()` installs flow's `flow.extension` operator installers
///   (brep/math/primitive/logic/dictionary/list/text) this artifact's own eval depends on;
///   `register_linked_flow_extension_installer` is the OTHER §6 function the mechanism's own census
///   names as excluded by design (flow's own extension registry, no declaration field). Idempotent
///   (`Once`-guarded), so calling it here preserves this plugin's prior eager-boot behavior exactly.
fn register_exports() {
    crate::apps::procedural2d::config::schema::register_app_schema();
    crate::apps::procedural3d::config::schema::register_app_schema();
    crate::artifacts::procedural3d::engine::register_dwg_mesh_bridge();
    crate::artifacts::procedural3d::engine::ensure_linked_flow_extensions();
}

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("procedural")
        .label("Procedural")
        .version("0.1.0")
        .setup(register_exports)
        .artifact(crate::artifacts::procedural2d::declaration())
        .artifact(crate::artifacts::procedural3d::declaration())
        .register_document_app::<crate::apps::procedural2d::Procedural2dPlayApp>(crate::apps::procedural2d::create_procedural2d_app())
        .register_document_app::<crate::apps::procedural3d::Procedural3dPlayApp>(crate::apps::procedural3d::create_procedural3d_app())
        .build()
}
