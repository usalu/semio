//! 🪆️ Subset root for `s.vcs.vcs@1/*` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM).
//! Exports `subset() -> SubsetDeclaration`, assembling the `🧬️schema`/`🚪️io`/`👁️viewer`/`✏️editor`/
//! `📚️examples` children — `crate::editor::vcs`/`crate::viewer::vcs` stay mounted at the plugin's
//! top-level `editor`/`viewer` modules (recipe §5 gotcha 1), not here.

use crate::editor::vcs as editor;
use crate::standards::v1::subsets::any::{io, schema};
use crate::viewer::vcs as viewer;
use crate::VCS_DIALECT;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;

fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::examples::demo::source()]).as_slice()
}

fn inference_descriptors() -> &'static [::framework_schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::framework_schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::vcs_artifact_inference_descriptor()]).as_slice()
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub fn subset<A: crate::VcsApplication>() -> SubsetDeclaration<A> {
    SubsetDeclaration {
        dialect: VCS_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::vcs_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::VcsViewer, A>(viewer::create_vcs_viewer()),
        editor: editor_surface::<editor::VcsPlayApp, A>(editor::create_vcs_app()),
        examples: examples(),
    }
}
