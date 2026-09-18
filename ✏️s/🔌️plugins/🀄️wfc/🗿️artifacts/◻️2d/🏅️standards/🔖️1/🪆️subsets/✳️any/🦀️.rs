//! 🪆️ Subset root for `s.wfc.wfc2d@1/*` — exports `subset() -> SubsetDeclaration`, assembling the
//! `🧬️schema`/`🚪️io`/`👁️viewer`/`✏️editor`/`📚️examples` children. `crate::editor::wfc2d`/
//! `crate::viewer::wfc2d` stay mounted at the crate's top-level `editor`/`viewer` modules, not here.

use crate::editor::wfc2d as editor;
use crate::standards::v1::subsets::any::{io, schema};
use crate::viewer::wfc2d as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;

fn examples() -> &'static [ExampleSource] {
    crate::examples::example_source_slice()
}

fn inference_descriptors() -> &'static [::semio_framework_schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: std::sync::OnceLock<Vec<::semio_framework_schema::ArtifactInferenceDescriptor>> = std::sync::OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::wfc2d_artifact_inference_descriptor()]).as_slice()
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub fn subset<PA: crate::ArtifactApps>() -> SubsetDeclaration<PA> {
    SubsetDeclaration {
        dialect: crate::WFC_2D_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::wfc2d_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::Wfc2dViewer, PA>(viewer::create_wfc2d_viewer()),
        editor: editor_surface::<editor::Wfc2dEditor, PA>(editor::create_wfc2d_editor()),
        examples: examples(),
    }
}
