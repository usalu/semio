//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("layout")
        .label("Layout")
        .version("0.1.0")
        .setup(crate::artifacts::layout::engine::register)
        .register_document_app::<crate::apps::layout::LayoutPlayApp>(crate::apps::layout::create_layout_app())
        .build()
}
