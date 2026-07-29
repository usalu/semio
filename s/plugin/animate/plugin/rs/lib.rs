//! 🎞️ Animate plugin — present tile play app bundled as a hot-swappable WASM plugin.

fn register_animate_present_exports() {
    semio_framework_os::register_2d_export_handlers("animate.present.deck", "animate", present_engine::animate_present_document_json_to_svg);
    semio_framework_os::register_dwg_import_handler("animate.present.deck", present_engine::animate_present_document_json_from_dwg);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<present_ui::AnimatePresentPlayApp>(present::PRESENT_DECK_SCHEMA);
}

semio_framework_plugin::semio_plugin! {
    id: "animate",
    label: "Animate",
    version: "0.1.0",
    setup: register_animate_present_exports,
    apps: [ present_ui::create_animate_present_app => present_ui::AnimatePresentPlayApp ],
}
