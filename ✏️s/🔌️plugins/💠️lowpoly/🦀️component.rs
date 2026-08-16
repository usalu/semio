//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old `.setup(engine::register)`
/// escape hatch; `.setup()` itself is gone (W1c) — `LowpolyPlayApp::app_schema()` now answers the
/// one thing it used to survive for, registered automatically by `register_document_app` below.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("lowpoly")
        .label("Lowpoly")
        .version("0.1.0")
        .artifact(crate::artifacts::lowpoly::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .document_app::<crate::apps::lowpoly::LowpolyPlayApp>(crate::apps::lowpoly::create_lowpoly_app())
        .try_build()
}
