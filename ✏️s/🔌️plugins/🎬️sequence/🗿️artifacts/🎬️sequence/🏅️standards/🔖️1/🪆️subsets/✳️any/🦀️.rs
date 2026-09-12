//! 🪆️ Subset root for `s.sequence.sequence@1/*` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-
//! MECHANISM). Exports `subset() -> SubsetDeclaration`, assembling the `🧬️schema`/`🚪️io`/`👁️viewer`/
//! `✏️editor`/`📚️examples` children — `crate::editor::sequence`/`crate::viewer::sequence` stay
//! mounted at the plugin's top-level `editor`/`viewer` modules (recipe §5 gotcha 1), not here.

use crate::editor::sequence as editor;
use crate::standards::v1::subsets::any::{io, schema};
use crate::viewer::sequence as viewer;
use crate::SEQUENCE_DIALECT;
use semio_framework_plugin::app::declarations::{editor_surface_with_members, viewer_surface_with_members, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;

fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::examples::demo::source()]).as_slice()
}

fn inference_descriptors() -> &'static [::framework_schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::framework_schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::sequence_artifact_inference_descriptor()]).as_slice()
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub fn subset<A: crate::SequenceApplication>() -> SubsetDeclaration<A> {
    SubsetDeclaration {
        dialect: SEQUENCE_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::sequence_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface_with_members::<viewer::SequenceViewer, semio_s_artifact_stdio_semio::SemioMembers, A>(viewer::create_sequence_viewer()),
        editor: editor_surface_with_members::<editor::SequencePlayApp, semio_s_artifact_stdio_semio::SemioMembers, A>(editor::create_sequence_app()),
        examples: examples(),
    }
}
