//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

fn register_exports() {
    crate::artifacts::procedural2d::engine::register();
    crate::artifacts::procedural3d::engine::register();
    crate::artifacts::procedural3d::engine::ensure_linked_flow_extensions();
}


/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("procedural")
        .label("Procedural")
        .version("0.1.0")
        .setup(register_exports)
        .register_document_app::<crate::apps::procedural2d::Procedural2dPlayApp>(crate::apps::procedural2d::create_procedural2d_app())
        .register_document_app::<crate::apps::procedural3d::Procedural3dPlayApp>(crate::apps::procedural3d::create_procedural3d_app())
        .build()
}
