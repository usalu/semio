//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("writer")
        .label("Writer")
        .version("0.1.0")
        .setup(crate::apps::writer::register)
        .document_app::<crate::apps::writer::WriterPlayApp>(crate::apps::writer::create_writer_app())
        .try_build()
}
