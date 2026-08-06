//! 📐️ `koordinator` pane — the demonstrator's entwerfen-mit-bestand coordination surface, served by
//! 📐️cad's `cad-play` app. Besides the mesh/document codecs every pane registers, this one also binds
//! the shared 3d BRep solid importers/exporters (obj/stl/step), which only the cad pane exposes.
//!
//! See <https://github.com/usalu/semio/issues/2510> for the bundle rationale.

use semio_framework_plugin::{GlbExporter, GlbImporter, PluginBundle};

use cad::apps::cad::{create_cad_app, CadPlayApp};
use cad::artifacts::cad::engine::{cad_document_from_dwg, cad_document_from_mesh, cad_mesh_from_document};
use cad::artifacts::cad::CAD_DOCUMENT_SCHEMA;

const CAD_KIND: &str = "3d.cad";
const CAD_FORMAT: &str = "cad";

/// 🔌️ Binds cad's solid/mesh/dwg codecs into the OS import/export registries and the app's document
/// codec into the plugin runtime.
pub fn register_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<CadPlayApp>(CAD_DOCUMENT_SCHEMA);
    semio_framework_os::register_solid_exporter(CAD_KIND, Box::new(semio_s_3d::brep::kernel::ObjSolidExporter));
    semio_framework_os::register_solid_exporter(CAD_KIND, Box::new(semio_s_3d::brep::kernel::StlSolidExporter));
    semio_framework_os::register_solid_exporter(CAD_KIND, Box::new(semio_s_3d::brep::kernel::StepSolidExporter));
    semio_framework_os::register_solid_importer(CAD_KIND, Box::new(semio_s_3d::brep::kernel::ObjSolidImporter));
    semio_framework_os::register_solid_importer(CAD_KIND, Box::new(semio_s_3d::brep::kernel::StlSolidImporter));
    semio_framework_os::register_solid_importer(CAD_KIND, Box::new(semio_s_3d::brep::kernel::StepSolidImporter));
    semio_framework_os::register_mesh_exporter(CAD_KIND, CAD_FORMAT, cad_mesh_from_document, Box::new(GlbExporter));
    semio_framework_os::register_mesh_importer(CAD_KIND, cad_document_from_mesh, Box::new(GlbImporter));
    semio_framework_os::register_mesh_dwg_export_handler(CAD_KIND, CAD_FORMAT, cad_mesh_from_document);
    semio_framework_os::register_dwg_import_handler(CAD_KIND, cad_document_from_dwg);
}

/// 🎪️ Adds the pane's app to the shared demonstrator bundle.
pub fn register_app(bundle: PluginBundle) -> PluginBundle {
    bundle.register_document_app(create_cad_app(), CadPlayApp::default)
}
