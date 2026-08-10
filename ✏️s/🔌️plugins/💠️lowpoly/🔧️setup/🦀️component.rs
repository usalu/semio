//! 🔧️ Setup facet for `💠️lowpoly` — codec/language/importer registration hooked via `.setup(...)`.

/// 🔌️ One call per `MeshExporter`/`MeshImporter` format so the OS workflow VFS auto-populates from
/// `required_media_formats`; also registers the
/// `ArtifactPack` codec so `.pack`/`.ops` sync/storage paths can encode/decode `LowpolySnapshot`.
pub fn register_lowpoly_exports() {
    crate::artifacts::lowpoly::engine::register();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::lowpoly::LowpolyPlayApp>(crate::artifacts::lowpoly::LOWPOLY_DOCUMENT_SCHEMA);
    semio_framework_os::register_mesh_exporter("3d.lowpoly", "lowpoly", crate::artifacts::lowpoly::engine::lowpoly_mesh_from_document, Box::new(semio_framework_plugin::ObjExporter));
    semio_framework_os::register_mesh_exporter("3d.lowpoly", "lowpoly", crate::artifacts::lowpoly::engine::lowpoly_mesh_from_document, Box::new(semio_framework_plugin::GlbExporter));
    semio_framework_os::register_mesh_exporter("3d.lowpoly", "lowpoly", crate::artifacts::lowpoly::engine::lowpoly_mesh_from_document, Box::new(semio_framework_plugin::StlExporter));
    semio_framework_os::register_mesh_dwg_export_handler("3d.lowpoly", "lowpoly", crate::artifacts::lowpoly::engine::lowpoly_mesh_from_document);
    semio_framework_os::register_mesh_importer("3d.lowpoly", crate::artifacts::lowpoly::engine::lowpoly_document_from_mesh, Box::new(semio_framework_plugin::ObjImporter));
    semio_framework_os::register_mesh_importer("3d.lowpoly", crate::artifacts::lowpoly::engine::lowpoly_document_from_mesh, Box::new(semio_framework_plugin::GlbImporter));
    semio_framework_os::register_mesh_importer("3d.lowpoly", crate::artifacts::lowpoly::engine::lowpoly_document_from_mesh, Box::new(semio_framework_plugin::StlImporter));
    semio_framework_os::register_mesh_dwg_import_handler("3d.lowpoly", crate::artifacts::lowpoly::engine::lowpoly_document_from_mesh);
    semio_framework_os::register_mesh_exporter("3d.mesh", "mesh", crate::artifacts::lowpoly::engine::mesh_from_mesh_document, Box::new(semio_framework_plugin::ObjExporter));
    semio_framework_os::register_mesh_exporter("3d.mesh", "mesh", crate::artifacts::lowpoly::engine::mesh_from_mesh_document, Box::new(semio_framework_plugin::GlbExporter));
    semio_framework_os::register_mesh_exporter("3d.mesh", "mesh", crate::artifacts::lowpoly::engine::mesh_from_mesh_document, Box::new(semio_framework_plugin::StlExporter));
    semio_framework_os::register_mesh_dwg_export_handler("3d.mesh", "mesh", crate::artifacts::lowpoly::engine::mesh_from_mesh_document);
    semio_framework_os::register_mesh_importer("3d.mesh", crate::artifacts::lowpoly::engine::mesh_document_from_mesh, Box::new(semio_framework_plugin::ObjImporter));
    semio_framework_os::register_mesh_importer("3d.mesh", crate::artifacts::lowpoly::engine::mesh_document_from_mesh, Box::new(semio_framework_plugin::GlbImporter));
    semio_framework_os::register_mesh_dwg_import_handler("3d.mesh", crate::artifacts::lowpoly::engine::mesh_document_from_mesh);
}
