//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("gis")
        .label("GIS")
        .version("0.1.0")
        .setup(crate::register_gis_exports)
        .register_document_app::<crate::apps::gis2d::Gis2dPlayApp>(crate::apps::gis2d::create_gis2d_app())
        .register_document_app::<crate::apps::gis3d::Gis3dPlayApp>(crate::apps::gis3d::create_gis3d_app())
        .build()
}
