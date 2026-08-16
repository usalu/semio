//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("animate")
        .label("Animate")
        .version("0.1.0")
        .artifact(crate::artifacts::present::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .document_app::<crate::apps::present::AnimatePresentPlayApp>(crate::apps::present::create_animate_present_app())
        .try_build()
}
