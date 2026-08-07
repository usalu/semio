//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("architect")
        .label("Architect")
        .version("0.1.0")
        .setup(crate::register_architect_exports)
        .register_document_app::<crate::apps::architect::ArchitectPlayApp>(crate::apps::architect::create_architect_app())
        .build()
}
