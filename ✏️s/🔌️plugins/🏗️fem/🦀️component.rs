//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("fem")
        .label("FEM")
        .version("0.1.0")
        .setup(crate::register_all_engines)
        .register_document_app::<crate::apps::fem2d::Fem2dPlayApp>(crate::apps::fem2d::create_fem2d_app())
        .register_document_app::<crate::apps::fem3d::Fem3dPlayApp>(crate::apps::fem3d::create_fem3d_app())
        .build()
}
