//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("mathematical")
        .label("Mathematical")
        .version("0.1.0")
        .setup(crate::artifacts::mathematical::engine::register)
        .register_document_app::<crate::apps::mathematical::MathematicalPlayApp>(crate::apps::mathematical::create_mathematical_app())
        .build()
}
