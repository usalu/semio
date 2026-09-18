//! 🪆️ Subset root for `s.wfc.grid2d@1/*` — `subset() -> SubsetDeclaration`, assembling the
//! `🧬️schema`/`🚪️io`/`👁️viewer`/`✏️editor`/`📚️examples` children. `editor`/`viewer` are read through
//! the crate-level `crate::editor::grid2d`/`crate::viewer::grid2d` mounts, never re-mounted here.

use crate::editor::grid2d as editor;
use crate::standards::v1::subsets::any::{io, schema};
use crate::viewer::grid2d as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;

fn examples() -> &'static [ExampleSource] {
    crate::examples::grid2d::example_source_slice()
}

fn inference_descriptors() -> &'static [::semio_framework_schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: std::sync::OnceLock<Vec<::semio_framework_schema::ArtifactInferenceDescriptor>> = std::sync::OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::grid2d_artifact_inference_descriptor()]).as_slice()
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub fn subset<PA: crate::ArtifactApps>() -> SubsetDeclaration<PA> {
    SubsetDeclaration {
        dialect: crate::WFC_GRID2D_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::grid2d_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::Grid2dViewer, PA>(viewer::create_grid2d_viewer()),
        editor: editor_surface::<editor::Grid2dEditor, PA>(editor::create_grid2d_editor()),
        examples: examples(),
    }
}
