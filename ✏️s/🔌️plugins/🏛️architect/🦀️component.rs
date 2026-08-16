//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("architect")
        .label("Architect")
        .version("0.1.0")
        .artifact(crate::artifacts::program::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .document_app::<crate::apps::architect::ArchitectPlayApp>(crate::apps::architect::create_architect_app())
        .try_build()
}
