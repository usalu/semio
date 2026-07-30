//! 🏺 Remodel plugin — photogrammetry/videogrammetry play app (video → watertight mesh) bundled as a
//! hot-swappable WASM plugin (thin bundle: host/media-codec registration + the `semio_plugin!` macro
//! block only). The actual app logic lives in the constitutional crates under `s/plugin/remodel/app/`:
//! `remodel` (document entities), `remodel_engine` (headless compute + exporters), `remodel_op`
//! (operation vocabulary), `remodel_dsl`/`remodel_pack`/`remodel_protocol` (surface wrappers + laws),
//! and `remodel_ui` (the `RemodelPlayApp` `DocumentApp` impl + manifest).

use remodel::REMODEL_DOCUMENT_SCHEMA;
use remodel_engine::{remodel_mesh_from_document, remodel_png_export, LasExporter, PlyExporter};
use remodel_ui::RemodelPlayApp;
use semio_framework_plugin::OsMediaFormat;

fn register_remodel_exports() {
    // 🗂️ Registers `RemodelScene`'s pack<->dsl codec under its real `document_schema()` string so
    // `framework/sync`'s `FolderEndpoint::Pack` (and any other schema-keyed caller) can print/parse
    // remodel documents without depending on this crate's concrete `Projection`/`Operation` types.
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<RemodelPlayApp>(REMODEL_DOCUMENT_SCHEMA);
    semio_framework_os::register_mesh_exporter("3d.remodel", "remodel", remodel_mesh_from_document, Box::new(semio_framework_plugin::ObjExporter));
    semio_framework_os::register_mesh_exporter("3d.remodel", "remodel", remodel_mesh_from_document, Box::new(semio_framework_plugin::GlbExporter));
    semio_framework_os::register_mesh_exporter("3d.remodel", "remodel", remodel_mesh_from_document, Box::new(semio_framework_plugin::StlExporter));
    semio_framework_os::register_mesh_exporter("3d.remodel", "remodel", remodel_mesh_from_document, Box::new(PlyExporter));
    semio_framework_os::register_mesh_exporter("3d.remodel", "remodel", remodel_mesh_from_document, Box::new(LasExporter));
    semio_framework_os::register_mesh_dwg_export_handler("3d.remodel", "remodel", remodel_mesh_from_document);
    semio_framework_os::register_os_media_export_handler("3d.remodel", OsMediaFormat::Png, remodel_png_export);
}

semio_framework_plugin::semio_plugin! {
    id: "remodel", label: "Remodel", version: "0.1.0",
    setup: register_remodel_exports,
    apps: [ remodel_ui::create_remodel_app => RemodelPlayApp ],
}
