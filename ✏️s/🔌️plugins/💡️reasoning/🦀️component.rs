//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("reasoning-mindmap")
        .label("Mindmap")
        .version("0.1.0")
        .artifact(crate::artifacts::wires::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .document_app::<crate::apps::wires::ReasoningWiresPlayApp>(crate::apps::wires::create_wires_app())
        .try_build()
}
