//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old `.setup(register_all_engines)`
/// escape hatch for both artifacts; `.setup()` itself is gone (W1c) — `Fem2dPlayApp::app_schema()`/
/// `Fem3dPlayApp::app_schema()` now answer the one thing it used to survive for, registered
/// automatically by each `register_document_app` call below.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("fem")
        .label("FEM")
        .version("0.1.0")
        .artifact(crate::artifacts::fem2d::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .artifact(crate::artifacts::fem3d::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .document_app::<crate::apps::fem2d::Fem2dPlayApp>(crate::apps::fem2d::create_fem2d_app())
        .document_app::<crate::apps::fem3d::Fem3dPlayApp>(crate::apps::fem3d::create_fem3d_app())
        .try_build()
}
