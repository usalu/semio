//! 📐️ Layout plugin — blueprint/preview document editor bundled as a hot-swappable WASM plugin.

fn register_layout_exports() {
    // 🫁️ GUESTSLIM: wires infinite_canvas's host-fetched typst font path (this crate builds
    // with `render` off) to the component `read-asset` import.
    infinite_canvas::host_asset::register_asset_reader(semio_framework_plugin::host_read_asset);
    // 🗂️ Registers `LayoutDocument`'s pack<->dsl codec under its real `document_schema()` string so
    // `framework/sync`'s `FolderEndpoint::Pack` (and any other schema-keyed caller) can print/parse
    // layout documents without depending on this crate's concrete `Projection`/`Operation` types.
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<layout_ui::LayoutPlayApp>(layout::LAYOUT_FIXTURE_SCHEMA);
    semio_framework_os::register_2d_export_handlers("2d.layout", "layout", layout_engine::layout_document_json_to_svg);
    semio_framework_os::register_dwg_import_handler("2d.layout", layout_engine::layout_document_json_from_dwg);
}

semio_framework_plugin::semio_plugin! {
    id: "layout",
    label: "Layout",
    version: "0.1.0",
    setup: register_layout_exports,
    apps: [ layout_ui::create_layout_app => layout_ui::LayoutPlayApp ],
}
