//! 🌐️ GIS plugin — 2D map + 3D terrain apps bundled as a hot-swappable WASM plugin.

fn register_gis_exports() {
    semio_framework_os::register_2d_export_handlers("2d.map", "gis2d", gis2d_engine::gis2d_document_json_to_svg);
    semio_framework_os::register_dwg_import_handler("2d.map", gis2d_engine::gis2d_document_json_from_dwg);
    // 🗂️ Sole native setup hook for the whole `gis` plugin bundle (`semio_plugin!`'s single
    // `setup: register_gis_exports`) — registers both document kinds' pack↔dsl codecs here since
    // `gis3d_ui` has no native registration fn of its own.
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<gis2d_ui::Gis2dPlayApp>(gis2d::GIS_MAP_SCHEMA);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<gis3d_ui::Gis3dPlayApp>(gis3d::GIS_3D_TERRAIN_SCHEMA);
}

semio_framework_plugin::semio_plugin! {
    id: "gis",
    label: "GIS",
    version: "0.1.0",
    setup: register_gis_exports,
    apps: [
        gis2d_ui::create_gis2d_app => gis2d_ui::Gis2dPlayApp,
        gis3d_ui::create_gis3d_app => gis3d_ui::Gis3dPlayApp,
    ],
}
