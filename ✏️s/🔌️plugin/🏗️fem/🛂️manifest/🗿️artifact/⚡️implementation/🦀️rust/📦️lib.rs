//! 🏗️ FEM WASM plugin: `fem2d-play` and `fem3d-play` apps registered as one hot-swappable component.

/// 🗂️ Registers `fem2d`/`fem3d`'s pack↔dsl codecs under their real `document_schema()` strings so
/// `framework/sync`'s `FolderEndpoint` (and any other schema-string-keyed caller) can print/parse
/// them without depending on `fem2d`/`fem3d`'s concrete `Projection`/`Operation` types.
fn register_fem_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<fem2d_ui::Fem2dPlayApp>(fem2d::FEM_2D_SCHEMA);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<fem3d_ui::Fem3dPlayApp>(fem3d::FEM_3D_SCHEMA);
}

semio_framework_plugin::semio_plugin! {
    id: "fem", label: "FEM", version: "0.1.0",
    setup: register_fem_exports,
    apps: [ fem2d_ui::create_fem2d_app => fem2d_ui::Fem2dPlayApp, fem3d_ui::create_fem3d_app => fem3d_ui::Fem3dPlayApp ],
}
