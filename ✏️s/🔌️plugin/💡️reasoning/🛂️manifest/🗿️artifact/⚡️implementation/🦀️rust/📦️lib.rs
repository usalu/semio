//! 🗺️ Mindmap plugin — WIRES app in a hot-swappable WASM plugin.
//! 🧠️ Mindmap Wires plugin — declarative WIRES play app bundled as a hot-swappable WASM plugin.

/// 🗂️ Registers `MindmapWiresDocument`'s pack↔dsl codec so `framework/sync`'s
/// `FolderEndpoint::Pack` (and any other schema-string-keyed caller) can print/parse it without
/// depending on this crate's concrete `Projection`/`Operation` types.
fn register_reasoning_mindmap_exports() {
    // 🫁️ GUESTSLIM: wires infinite_canvas's host-fetched typst font path (this crate builds
    // with `render` off) to the component `read-asset` import.
    infinite_canvas::host_asset::register_asset_reader(semio_framework_plugin::host_read_asset);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<reasoning_wires_ui::ReasoningWiresPlayApp>(reasoning_wires::MINDMAP_WIRES_SCHEMA);
}

semio_framework_plugin::semio_plugin! {
    id: "reasoning-mindmap", label: "Mindmap", version: "0.1.0",
    setup: register_reasoning_mindmap_exports,
    apps: [ reasoning_wires_ui::create_wires_app => reasoning_wires_ui::ReasoningWiresPlayApp ],
}
