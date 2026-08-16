//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("vcs")
        .label("VCS")
        .version("0.1.0")
        .artifact(crate::artifacts::vcs::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .document_app::<crate::apps::vcs::VcsPlayApp>(crate::apps::vcs::create_vcs_app())
        .try_build()
}
