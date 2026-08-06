//! 🗺️ `verfolgen` pane — the demonstrator's entwerfen-mit-bestand tracking surface, served by 🌍️gis's
//! `gis2d-play` app. Only the 2d half of gis's host wiring is registered here: the pane boots
//! `gis2d-play` exclusively, so `gis`'s 3d terrain app is never reached through this bundle.
//!
//! See <https://github.com/usalu/semio/issues/2510> for the bundle rationale.

use semio_framework_plugin::PluginBundle;

use gis::apps::gis2d::{create_gis2d_app, Gis2dPlayApp};
use gis::artifacts::gismap::engine::{gis2d_document_json_from_dwg, gis2d_document_json_to_svg};
use gis::artifacts::gismap::GIS_MAP_SCHEMA;

const GIS_MAP_KIND: &str = "2d.map";
const GIS_MAP_FORMAT: &str = "gis2d";

/// 🔌️ Binds gis's 2d svg/dwg codecs into the OS export registries and the app's document codec into
/// the plugin runtime.
pub fn register_exports() {
    semio_framework_os::register_2d_export_handlers(GIS_MAP_KIND, GIS_MAP_FORMAT, gis2d_document_json_to_svg);
    semio_framework_os::register_dwg_import_handler(GIS_MAP_KIND, gis2d_document_json_from_dwg);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<Gis2dPlayApp>(GIS_MAP_SCHEMA);
}

/// 🎪️ Adds the pane's app to the shared demonstrator bundle.
pub fn register_app(bundle: PluginBundle) -> PluginBundle {
    bundle.register_document_app(create_gis2d_app(), || Gis2dPlayApp)
}
