//! 🧮 Combined mathematical framework playground — graph algorithms and computational geometry as one hot-swappable WASM plugin.

fn register_mathematical_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<mathematical_ui::MathematicalPlayApp>("semio.mathematical/v1");
}

semio_framework_plugin::semio_plugin! {
    id: "mathematical", label: "Mathematical", version: "0.1.0",
    setup: register_mathematical_exports,
    apps: [ mathematical_ui::create_mathematical_app => mathematical_ui::MathematicalPlayApp ],
}
