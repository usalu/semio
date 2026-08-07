//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("lowpoly")
        .label("Lowpoly")
        .version("0.1.0")
        .setup(crate::register_lowpoly_exports)
        .register_document_app::<crate::apps::lowpoly::LowpolyPlayApp>(crate::apps::lowpoly::create_lowpoly_app())
        .build()
}
