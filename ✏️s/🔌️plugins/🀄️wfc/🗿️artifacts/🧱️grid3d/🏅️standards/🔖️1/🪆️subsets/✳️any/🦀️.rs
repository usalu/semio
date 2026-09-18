//! 🪆️ Subset root for `s.wfc.grid3d@1/*` — the ONLY registration channel for this subset's
//! schema/io/viewer/editor/examples rows, assembled exactly like `s.puzzle.puzzle2d`'s.
//! `crate::editor::grid3d`/`crate::viewer::grid3d` stay mounted at the crate's top-level
//! `editor`/`viewer` modules, never here.

use crate::editor::grid3d as editor;
use crate::standards::v1::subsets::any::schema;
use crate::viewer::grid3d as viewer;
use crate::{Grid3dMutation, Grid3dSnapshot, WFC_GRID3D_DIALECT, WFC_GRID3D_DOCUMENT_SCHEMA};
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, IoDeclaration, LanguagePair, NativeCodecs, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;

fn examples() -> &'static [ExampleSource] {
    crate::examples::example_source_slice()
}

fn inference_descriptors() -> &'static [::semio_framework_schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::semio_framework_schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::grid3d_artifact_inference_descriptor()]).as_slice()
}

/// 🚪️ This subset's io surface. `entries` is empty on purpose: `s.wfc.grid3d` ships no foreign
/// format hop of its own — the stdio text round-trip is the plugin-level composer, and the native
/// pair below is the real, complete channel. `pilot_languages()`' order is fixed by that function's
/// own literal `vec![document, op, pack, spr]`.
pub fn io() -> IoDeclaration {
    let languages = crate::pilot_languages();
    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: Some(&languages[0]), binary: Some(&languages[2]) },
            diff: LanguagePair { text: None, binary: None },
            mutations: LanguagePair { text: Some(&languages[1]), binary: Some(&languages[3]) },
            inferences: None,
            codec: store::ArtifactCodec::of::<Grid3dSnapshot, Grid3dMutation>(WFC_GRID3D_DOCUMENT_SCHEMA.to_string()),
        },
        entries: &[],
    }
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub fn subset<PA: crate::ArtifactApps>() -> SubsetDeclaration<PA> {
    SubsetDeclaration {
        dialect: WFC_GRID3D_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::grid3d_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io(),
        viewer: viewer_surface::<viewer::Grid3dViewer, PA>(viewer::create_grid3d_viewer()),
        editor: editor_surface::<editor::Grid3dEditor, PA>(editor::create_grid3d_editor()),
        examples: examples(),
    }
}
