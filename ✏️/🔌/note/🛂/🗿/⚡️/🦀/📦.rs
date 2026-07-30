//! 📝 Note plugin — infinite canvas note board bundled as a hot-swappable WASM plugin.

fn register_note_exports() {
    semio_framework_os::register_2d_export_handlers("2d.note", "note", note_engine::note_document_json_to_svg);
    semio_framework_os::register_dwg_import_handler("2d.note", note_engine::note_document_json_from_dwg);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<note_ui::NotePlayApp>(note::NOTE_DOCUMENT_SCHEMA);
}

semio_framework_plugin::semio_plugin! {
    id: "note", label: "Note", version: "0.1.0",
    setup: register_note_exports,
    apps: [ note_ui::create_note_app => note_ui::NotePlayApp ],
}
