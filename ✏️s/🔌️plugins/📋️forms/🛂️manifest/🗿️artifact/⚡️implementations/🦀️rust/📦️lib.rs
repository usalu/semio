//! 📋️ Forms plugin — declarative forms play app bundled as a hot-swappable WASM plugin.

/// 🗂️ Registers `FormSpec`'s pack<->dsl codec under its real `document_schema()` string so
/// `framework/sync`'s `FolderEndpoint::Pack` (and any other schema-keyed caller) can print/parse
/// forms documents without depending on this crate's concrete `Projection`/`Operation` types.
fn register_forms_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<forms_ui::FormsPlayApp>(forms::FORMS_DOCUMENT_SCHEMA);
}

semio_framework_plugin::semio_plugin! {
    id: "forms",
    label: "Forms",
    version: "0.1.0",
    setup: register_forms_exports,
    apps: [ forms_ui::create_forms_app => forms_ui::FormsPlayApp ],
}
