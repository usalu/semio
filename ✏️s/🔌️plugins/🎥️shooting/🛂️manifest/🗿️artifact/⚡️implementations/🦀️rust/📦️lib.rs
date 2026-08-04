//! 📸️ Shooting plugin — icon studio with scene + preview windows bundled as a hot-swappable WASM plugin.

fn register_shooting_exports() {
    semio_framework_os::register_2d_export_handlers("2d.shooting", "shooting", shooting_engine::shooting_document_json_to_svg);
    semio_framework_os::register_dwg_import_handler("2d.shooting", shooting_engine::shooting_document_json_from_dwg);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<shooting_ui::ShootingPlayApp>(shooting::SHOOTING_FIXTURE_SCHEMA);
}

semio_framework_plugin::semio_plugin! {
    id: "shooting", label: "Shooting", version: "0.1.0",
    setup: register_shooting_exports,
    apps: [ shooting_ui::create_shooting_app => shooting_ui::ShootingPlayApp ],
}
