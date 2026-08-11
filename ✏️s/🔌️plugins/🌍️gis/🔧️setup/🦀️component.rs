//! 🔧️ Setup facet for `🌍️gis` — codec/language/importer registration hooked via `.setup(...)`.

/// 🔌️ Plugin `setup:` hook — register GIS host exports (languages/codecs/app schema) once at load.
pub fn register_gis_exports() {
    crate::artifacts::gismap::engine::register_pilot_languages();
    crate::artifacts::gisterrain::engine::register_pilot_languages();

    crate::apps::gis2d::config::schema::register_app_schema();
    crate::apps::gis3d::config::schema::register_app_schema();
}
