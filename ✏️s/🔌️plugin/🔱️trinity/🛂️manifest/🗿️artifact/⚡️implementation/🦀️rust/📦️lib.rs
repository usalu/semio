//! 🔺️ Trinity plugin — Jack and Rewrite apps in one hot-swappable WASM plugin.

//#region 🔖️Bundle
/// 🗂️ Registers this crate's two document kinds' pack↔dsl codecs so `framework/sync`'s
/// `FolderEndpoint::Pack` (and any other schema-string-keyed caller) can print/parse them without
/// depending on `trinity_ram`/`trinity_rewrite`'s concrete `Projection`/`Operation` types.
fn register_trinity_exports() {
    // 🫁️ GUESTSLIM: wires infinite_canvas's host-fetched typst font path (this crate builds
    // with `render` off) to the component `read-asset` import.
    infinite_canvas::host_asset::register_asset_reader(semio_framework_plugin::host_read_asset);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<trinity_jack_ui::TrinityJackPlayApp>(trinity_ram::TRINITY_GRAPH_SCHEMA);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<trinity_rewrite_ui::TrinityRewritePlayApp>(trinity_rewrite::REWRITE_RULE_SCHEMA);
}

semio_framework_plugin::semio_plugin! {
    id: "trinity",
    label: "Trinity",
    version: "0.1.0",
    setup: register_trinity_exports,
    apps: [
        trinity_jack_ui::create_trinity_jack_app => trinity_jack_ui::TrinityJackPlayApp,
        trinity_rewrite_ui::create_rewrite_app => trinity_rewrite_ui::TrinityRewritePlayApp,
    ]
}
//#endregion 🔖️Bundle
