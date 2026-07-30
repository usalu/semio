//! 🌊 Flow plugin — declarative flow play app bundled as a hot-swappable WASM plugin.

//#region 🔖PackCodec
/// 🗂️ Registers `FlowFixture`'s pack↔dsl codec under `FLOW_DOCUMENT_SCHEMA` so `framework/sync`'s
/// folder endpoints and any other schema-string-keyed caller can print/parse flow documents.
fn register_flow_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<flow_ui::FlowPlayApp>(flow::FLOW_DOCUMENT_SCHEMA);
}
//#endregion 🔖PackCodec

semio_framework_plugin::semio_plugin! {
    id: "flow", label: "Flow", version: "0.1.0",
    setup: register_flow_exports,
    apps: [ flow_ui::create_flow_app => flow_ui::FlowPlayApp ],
}
