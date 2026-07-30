//! 🔀 DAG plugin — declarative DAG play app bundled as a hot-swappable WASM plugin.

//#region 🔖PackCodec
/// 🗂️ Registers `DagDocument`'s pack<->dsl codec under its real `document_schema()` string so
/// `framework/sync`'s `FolderEndpoint::Pack` (and any other schema-keyed caller) can print/parse
/// DAG documents without depending on this crate's concrete `Projection`/`Operation` types.
fn register_dag_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<dag_ui::DagPlayApp>(dag::DAG_DOCUMENT_SCHEMA);
}
//#endregion 🔖PackCodec

semio_framework_plugin::semio_plugin! {
    id: "dag", label: "DAG", version: "0.1.0",
    setup: register_dag_exports,
    apps: [ dag_ui::create_dag_app => dag_ui::DagPlayApp ],
}
