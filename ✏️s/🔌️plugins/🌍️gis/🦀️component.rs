//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Registers GIS host exports (languages/app schema) once at plugin load — folded in from the
/// dissolved `🔧️setup` facet per APA (`26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`): all
/// registration belongs to the plugin's own load path, never a standalone setup facet, and this
/// fan-out spans both owned artifacts (`gismap`, `gisterrain`) plus both apps' schemas, so no
/// single artifact `⚙️engine` is the sole owner.
fn register_gis_exports() {
    crate::artifacts::gismap::engine::register_pilot_languages();
    crate::artifacts::gisterrain::engine::register_pilot_languages();

    crate::apps::gis2d::config::schema::register_app_schema();
    crate::apps::gis3d::config::schema::register_app_schema();
}

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("gis")
        .label("GIS")
        .version("0.1.0")
        .setup(register_gis_exports)
        .register_document_app::<crate::apps::gis2d::Gis2dPlayApp>(crate::apps::gis2d::create_gis2d_app())
        .register_document_app::<crate::apps::gis3d::Gis3dPlayApp>(crate::apps::gis3d::create_gis3d_app())
        .build()
}
