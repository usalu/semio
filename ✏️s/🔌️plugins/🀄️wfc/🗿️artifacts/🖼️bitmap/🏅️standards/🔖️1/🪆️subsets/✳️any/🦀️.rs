//! 🪆️ Subset root for `s.wfc.bitmap@1/*`. Exports `subset() -> SubsetDeclaration`, assembling the
//! `🧬️schema`/`🚪️io`/`👁️viewer`/`✏️editor`/`📚️examples` children. `crate::editor::bitmap` and
//! `crate::viewer::bitmap` stay mounted at the crate's top-level `editor`/`viewer` modules, not
//! here, so a viewer file never reaches its own surface through this subset root.

use crate::editor::bitmap as editor;
use crate::standards::v1::subsets::any::{io, schema};
use crate::viewer::bitmap as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use std::sync::OnceLock;

/// 💡️ `::semio_framework_schema::` (the extern crate) vs the bare `schema` local import (this
/// subset's own schema module) — the two share a name, only the leading path disambiguates.
fn inference_descriptors() -> &'static [::semio_framework_schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::semio_framework_schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::bitmap_artifact_inference_descriptor()]).as_slice()
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub fn subset<PA: crate::ArtifactApps>() -> SubsetDeclaration<PA> {
    SubsetDeclaration {
        dialect: crate::WFC_BITMAP_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::bitmap_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::BitmapViewer, PA>(viewer::create_bitmap_viewer()),
        editor: editor_surface::<editor::BitmapEditor, PA>(editor::create_bitmap_editor()),
        examples: crate::examples::example_source_slice(),
    }
}
