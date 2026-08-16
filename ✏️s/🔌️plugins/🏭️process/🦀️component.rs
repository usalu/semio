//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("process")
        .label("Process")
        .version("0.1.0")
        .setup(crate::apps::process3d::register)
        .document_app::<crate::apps::process3d::Process3dPlayApp>(crate::apps::process3d::create_process3d_app())
        .try_build()
}
