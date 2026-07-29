//! 🗂️ VCS plugin — declarative version-control play app bundled as a hot-swappable WASM plugin.

fn register_vcs_exports() {
    // 🗂️ Registers `VcsDemoProjection`'s pack<->dsl codec under its real `document_schema()` string
    // so `framework/sync`'s `FolderEndpoint::Pack` (and any other schema-keyed caller) can print/parse
    // vcs-play documents without depending on this crate's concrete `Projection`/`Operation` types.
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<vcs_ui::VcsPlayApp>(vcs::VCS_DEMO_SCHEMA);
}

semio_framework_plugin::semio_plugin! {
    id: "vcs", label: "VCS", version: "0.1.0",
    setup: register_vcs_exports,
    apps: [ vcs_ui::create_vcs_app => vcs_ui::VcsPlayApp ],
}
