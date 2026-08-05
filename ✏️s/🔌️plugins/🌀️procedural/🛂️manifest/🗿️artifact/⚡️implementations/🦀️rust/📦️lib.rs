//! 🔧️ Procedural plugin — 2D and 3D flow apps in one hot-swappable WASM plugin.

fn register_procedural_exports() {
    semio_framework_os::register_2d_export_handlers("2d.procedural", "procedural2d", procedural_2d_engine::procedural2d_document_json_to_svg);
    semio_framework_os::register_dwg_import_handler("2d.procedural", procedural_2d_engine::procedural2d_document_from_dwg);
    // 📦️ Registers `Procedural2dDocument`'s pack<->dsl codec so `framework/sync`'s `FolderEndpoint`
    // can print/parse `.procedural2d` packs without depending on this crate's concrete types.
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<procedural_2d_ui::Procedural2dPlayApp>(procedural_2d::PROCEDURAL_2D_SCHEMA);

    semio_framework_os::register_mesh_exporter("3d.procedural", "procedural", procedural_3d_engine::procedural3d_mesh_from_document, Box::new(semio_framework_plugin::ObjExporter));
    semio_framework_os::register_mesh_exporter("3d.procedural", "procedural", procedural_3d_engine::procedural3d_mesh_from_document, Box::new(semio_framework_plugin::GlbExporter));
    semio_framework_os::register_mesh_exporter("3d.procedural", "procedural", procedural_3d_engine::procedural3d_mesh_from_document, Box::new(semio_framework_plugin::StlExporter));
    semio_framework_os::register_mesh_dwg_export_handler("3d.procedural", "procedural", procedural_3d_engine::procedural3d_mesh_from_document);
    semio_framework_os::register_mesh_importer("3d.procedural", procedural_3d_engine::procedural3d_document_from_mesh, Box::new(semio_framework_plugin::ObjImporter));
    semio_framework_os::register_mesh_importer("3d.procedural", procedural_3d_engine::procedural3d_document_from_mesh, Box::new(semio_framework_plugin::GlbImporter));
    semio_framework_os::register_mesh_importer("3d.procedural", procedural_3d_engine::procedural3d_document_from_mesh, Box::new(semio_framework_plugin::StlImporter));
    semio_framework_os::register_mesh_dwg_import_handler("3d.procedural", procedural_3d_engine::procedural3d_document_from_mesh);
    // 📦️ Registers `Procedural3dDocument`'s pack<->dsl codec so `framework/sync`'s `FolderEndpoint`
    // can print/parse `.procedural3d` packs without depending on this crate's concrete types.
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<procedural_3d_ui::Procedural3dPlayApp>(procedural_3d::PROCEDURAL_3D_SCHEMA);
}

semio_framework_plugin::semio_plugin! {
    id: "procedural",
    label: "Procedural",
    version: "0.1.0",
    setup: register_procedural_exports,
    apps: [
        procedural_2d_ui::create_procedural2d_app => procedural_2d_ui::Procedural2dPlayApp,
        procedural_3d_ui::create_procedural3d_app => procedural_3d_ui::Procedural3dPlayApp,
    ],
}
