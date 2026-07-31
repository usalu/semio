//! 📏️ CAD plugin — spatial model play app bundled as a hot-swappable WASM plugin.

fn cad_mesh_from_document(doc: &serde_json::Value) -> Result<semio_framework_plugin::MeshData, String> {
    cad_document_engine::cad_mesh_from_document(doc)
}

fn cad_document_from_dwg(drawing: &semio_framework_core::DwgDrawing) -> Result<serde_json::Value, String> {
    cad_document_engine::cad_document_from_dwg(drawing)
}

fn cad_document_from_mesh(mesh: &semio_framework_plugin::MeshData) -> Result<serde_json::Value, String> {
    cad_document_engine::cad_document_from_mesh(mesh)
}

fn register_cad_exports() {
    // 📦️ pack binary codec for `CadScene` (`CadPlayApp::document_schema()` == `CAD_DOCUMENT_SCHEMA`).
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<cad_document_ui::CadPlayApp>(cad_document::CAD_DOCUMENT_SCHEMA);
    semio_framework_os::register_solid_exporter("3d.cad", Box::new(kernel_3d_brepkit::ObjSolidExporter));
    semio_framework_os::register_solid_exporter("3d.cad", Box::new(kernel_3d_brepkit::StlSolidExporter));
    semio_framework_os::register_solid_exporter("3d.cad", Box::new(kernel_3d_brepkit::StepSolidExporter));
    semio_framework_os::register_solid_importer("3d.cad", Box::new(kernel_3d_brepkit::ObjSolidImporter));
    semio_framework_os::register_solid_importer("3d.cad", Box::new(kernel_3d_brepkit::StlSolidImporter));
    semio_framework_os::register_solid_importer("3d.cad", Box::new(kernel_3d_brepkit::StepSolidImporter));
    semio_framework_os::register_mesh_exporter("3d.cad", "cad", cad_mesh_from_document, Box::new(semio_framework_plugin::GlbExporter));
    semio_framework_os::register_mesh_importer("3d.cad", cad_document_from_mesh, Box::new(semio_framework_plugin::GlbImporter));
    semio_framework_os::register_mesh_dwg_export_handler("3d.cad", "cad", cad_mesh_from_document);
    semio_framework_os::register_dwg_import_handler("3d.cad", cad_document_from_dwg);
}

semio_framework_plugin::semio_plugin! {
    id: "cad",
    label: "CAD",
    version: "0.1.0",
    setup: register_cad_exports,
    apps: [ cad_document_ui::create_cad_app => cad_document_ui::CadPlayApp ],
}
