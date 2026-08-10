//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("cad")
        .label("CAD")
        .version("0.1.0")
        .setup(crate::artifacts::cad::engine::register)
        .register_document_app::<crate::apps::cad::CadPlayApp>(crate::apps::cad::create_cad_app())
        .build()
}
