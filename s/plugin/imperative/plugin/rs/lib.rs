//! ⚡ Imperative plugin — declarative imperative play app bundled as a hot-swappable WASM plugin.

fn register_imperative_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<imperative_ui::ImperativePlayApp>(imperative_ui::IMPERATIVE_DOCUMENT_SCHEMA);
}

semio_framework_plugin::semio_plugin! {
    id: "imperative",
    label: "Imperative",
    version: "0.1.0",
    setup: register_imperative_exports,
    apps: [ imperative_ui::create_imperative_app => imperative_ui::ImperativePlayApp ],
}
