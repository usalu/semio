//! 🌱️ `generator` pane — the demonstrator's entwerfen-mit-bestand generator surface, served by
//! 🌀️procedural's `procedural3d-play` app. Only the 3d half of procedural's host wiring is registered
//! here: the pane boots `procedural3d-play` exclusively, so `procedural`'s 2d app is never reached
//! through this bundle.
//!
//! See <https://github.com/usalu/semio/issues/2510> for the bundle rationale.

use semio_framework_plugin::{GlbExporter, GlbImporter, ObjExporter, ObjImporter, Plugin, StlExporter, StlImporter};

use procedural::apps::procedural3d::{create_procedural3d_app, Procedural3dPlayApp};
use procedural::artifacts::procedural3d::engine::{procedural3d_document_from_mesh, procedural3d_mesh_from_document};
use procedural::artifacts::procedural3d::PROCEDURAL_3D_SCHEMA;

const PROCEDURAL_3D_KIND: &str = "3d.procedural";
const PROCEDURAL_3D_FORMAT: &str = "procedural";

/// 🔌️ Binds procedural's 3d mesh codecs into the OS import/export registries and the app's document
/// codec into the plugin runtime, so the pane's meshes and documents round-trip through the host.
pub fn register_exports() {
    semio_framework_os::register_mesh_exporter(PROCEDURAL_3D_KIND, PROCEDURAL_3D_FORMAT, procedural3d_mesh_from_document, Box::new(ObjExporter));
    semio_framework_os::register_mesh_exporter(PROCEDURAL_3D_KIND, PROCEDURAL_3D_FORMAT, procedural3d_mesh_from_document, Box::new(GlbExporter));
    semio_framework_os::register_mesh_exporter(PROCEDURAL_3D_KIND, PROCEDURAL_3D_FORMAT, procedural3d_mesh_from_document, Box::new(StlExporter));
    semio_framework_os::register_mesh_dwg_export_handler(PROCEDURAL_3D_KIND, PROCEDURAL_3D_FORMAT, procedural3d_mesh_from_document);
    semio_framework_os::register_mesh_importer(PROCEDURAL_3D_KIND, procedural3d_document_from_mesh, Box::new(ObjImporter));
    semio_framework_os::register_mesh_importer(PROCEDURAL_3D_KIND, procedural3d_document_from_mesh, Box::new(GlbImporter));
    semio_framework_os::register_mesh_importer(PROCEDURAL_3D_KIND, procedural3d_document_from_mesh, Box::new(StlImporter));
    semio_framework_os::register_mesh_dwg_import_handler(PROCEDURAL_3D_KIND, procedural3d_document_from_mesh);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<Procedural3dPlayApp>(PROCEDURAL_3D_SCHEMA);
}

/// 🎪️ Adds the pane's app to the shared demonstrator bundle.
pub fn register_app(bundle: Plugin) -> Plugin {
    bundle.register_document_app::<Procedural3dPlayApp>(create_procedural3d_app())
}
