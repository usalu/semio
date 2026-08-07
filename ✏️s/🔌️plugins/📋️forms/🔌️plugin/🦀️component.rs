//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("forms")
        .label("Forms")
        .version("0.1.0")
        .setup(crate::artifacts::forms::engine::register)
        .register_document_app::<crate::apps::forms::FormsPlayApp>(crate::apps::forms::create_forms_app())
        .build()
}
