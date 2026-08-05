//! 🔗️ Sequence plugin — declarative sequence play app bundled as a hot-swappable WASM plugin.

fn register_sequence_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<sequence_ui::SequencePlayApp>(sequence::SEQUENCE_FIXTURE_SCHEMA);
}

semio_framework_plugin::semio_plugin! {
    id: "sequence",
    label: "Sequence",
    version: "0.1.0",
    setup: register_sequence_exports,
    apps: [ sequence_ui::create_sequence_app => sequence_ui::SequencePlayApp ],
}
