//! 🔺 Lowpoly plugin — mesh + paint editor bundled as a hot-swappable WASM plugin.

/// 🔌 One call per `MeshExporter`/`MeshImporter` format so the OS workflow VFS auto-populates from
/// `required_os_media_export_formats`/`required_os_media_import_formats`; also registers the
/// `DocumentPack` codec so `.pack`/`.ops` sync/storage paths can encode/decode `LowpolyProjection`.
fn register_lowpoly_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<lowpoly_ui::LowpolyPlayApp>(lowpoly::LOWPOLY_DOCUMENT_SCHEMA);
    semio_framework_os::register_mesh_exporter("3d.lowpoly", "lowpoly", lowpoly_engine::lowpoly_mesh_from_document, Box::new(semio_framework_plugin::ObjExporter));
    semio_framework_os::register_mesh_exporter("3d.lowpoly", "lowpoly", lowpoly_engine::lowpoly_mesh_from_document, Box::new(semio_framework_plugin::GlbExporter));
    semio_framework_os::register_mesh_exporter("3d.lowpoly", "lowpoly", lowpoly_engine::lowpoly_mesh_from_document, Box::new(semio_framework_plugin::StlExporter));
    semio_framework_os::register_mesh_dwg_export_handler("3d.lowpoly", "lowpoly", lowpoly_engine::lowpoly_mesh_from_document);
    semio_framework_os::register_mesh_importer("3d.lowpoly", lowpoly_engine::lowpoly_document_from_mesh, Box::new(semio_framework_plugin::ObjImporter));
    semio_framework_os::register_mesh_importer("3d.lowpoly", lowpoly_engine::lowpoly_document_from_mesh, Box::new(semio_framework_plugin::GlbImporter));
    semio_framework_os::register_mesh_importer("3d.lowpoly", lowpoly_engine::lowpoly_document_from_mesh, Box::new(semio_framework_plugin::StlImporter));
    semio_framework_os::register_mesh_dwg_import_handler("3d.lowpoly", lowpoly_engine::lowpoly_document_from_mesh);
    semio_framework_os::register_mesh_exporter("3d.mesh", "mesh", lowpoly_engine::mesh_from_mesh_document, Box::new(semio_framework_plugin::ObjExporter));
    semio_framework_os::register_mesh_exporter("3d.mesh", "mesh", lowpoly_engine::mesh_from_mesh_document, Box::new(semio_framework_plugin::GlbExporter));
    semio_framework_os::register_mesh_exporter("3d.mesh", "mesh", lowpoly_engine::mesh_from_mesh_document, Box::new(semio_framework_plugin::StlExporter));
    semio_framework_os::register_mesh_dwg_export_handler("3d.mesh", "mesh", lowpoly_engine::mesh_from_mesh_document);
    semio_framework_os::register_mesh_importer("3d.mesh", lowpoly_engine::mesh_document_from_mesh, Box::new(semio_framework_plugin::ObjImporter));
    semio_framework_os::register_mesh_importer("3d.mesh", lowpoly_engine::mesh_document_from_mesh, Box::new(semio_framework_plugin::GlbImporter));
    semio_framework_os::register_mesh_importer("3d.mesh", lowpoly_engine::mesh_document_from_mesh, Box::new(semio_framework_plugin::StlImporter));
    semio_framework_os::register_mesh_dwg_import_handler("3d.mesh", lowpoly_engine::mesh_document_from_mesh);
}

semio_framework_plugin::semio_plugin! {
    id: "lowpoly", label: "Lowpoly", version: "0.1.0",
    setup: register_lowpoly_exports,
    apps: [ lowpoly_ui::create_lowpoly_app => lowpoly_ui::LowpolyPlayApp ],
}
