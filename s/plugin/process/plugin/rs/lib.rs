//! 🪚 Process plugin — subtractive/additive processing simulation in one hot-swappable WASM plugin.

use semio_framework_plugin::MeshData;
use serde_json::Value;

fn process3d_mesh_from_document(doc: &Value) -> Result<MeshData, String> {
    let document: process_3d::Process3dDocument = serde_json::from_value(doc.clone()).map_err(|error| error.to_string())?;
    process_3d_engine::processed_mesh(&document).ok_or_else(|| "process3d: kernel replay failed".to_string())
}

fn process3d_document_from_mesh(_mesh: &MeshData) -> Result<Value, String> {
    Err("process3d: mesh import not supported".into())
}

fn register_process3d_exports() {
    semio_framework_os::register_mesh_exporter("3d.process", "process", process3d_mesh_from_document, Box::new(semio_framework_plugin::ObjExporter));
    semio_framework_os::register_mesh_exporter("3d.process", "process", process3d_mesh_from_document, Box::new(semio_framework_plugin::GlbExporter));
    semio_framework_os::register_mesh_exporter("3d.process", "process", process3d_mesh_from_document, Box::new(semio_framework_plugin::StlExporter));
    semio_framework_os::register_mesh_dwg_export_handler("3d.process", "process", process3d_mesh_from_document);
    semio_framework_os::register_mesh_dwg_import_handler("3d.process", process3d_document_from_mesh);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<process_3d_ui::Process3dPlayApp>(process_3d::PROCESS_3D_SCHEMA);
}

semio_framework_plugin::semio_plugin! {
    id: "process", label: "Process", version: "0.1.0",
    setup: register_process3d_exports,
    apps: [ process_3d_ui::create_process3d_app => process_3d_ui::Process3dPlayApp ],
}
