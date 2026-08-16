//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

fn register_exports() {
    crate::apps::sequence::register();
    crate::apps::sequence::config::schema::register_app_schema();
}

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("sequence")
        .label("Sequence")
        .version("0.1.0")
        .setup(register_exports)
        .document_app::<crate::apps::sequence::SequencePlayApp>(crate::apps::sequence::create_sequence_app())
        .try_build()
}
