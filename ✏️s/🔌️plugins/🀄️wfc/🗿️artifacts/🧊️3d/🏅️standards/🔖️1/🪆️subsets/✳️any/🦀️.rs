//! 🪆️ Subset root for `s.wfc.wfc3d@1/*` — `subset() -> SubsetDeclaration`, assembling the
//! `🧬️schema`/`🚪️io`/`👁️viewer`/`✏️editor`/`📚️examples` children. `crate::editor::wfc3d` and
//! `crate::viewer::wfc3d` stay mounted at the crate's top-level `editor`/`viewer` modules, not here.

use crate::editor::wfc3d as editor;
use crate::standards::v1::subsets::any::{io, schema};
use crate::viewer::wfc3d as viewer;
use crate::WFC3D_DIALECT;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;

fn examples() -> &'static [ExampleSource] {
    crate::examples::example_source_slice()
}

fn inference_descriptors() -> &'static [::semio_framework_schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::semio_framework_schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::wfc3d_artifact_inference_descriptor()]).as_slice()
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub fn subset<PA: crate::ArtifactApps>() -> SubsetDeclaration<PA> {
    SubsetDeclaration {
        dialect: WFC3D_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::wfc3d_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::Wfc3dViewer, PA>(viewer::create_wfc3d_viewer()),
        editor: editor_surface::<editor::Wfc3dEditor, PA>(editor::create_wfc3d_editor()),
        examples: examples(),
    }
}
