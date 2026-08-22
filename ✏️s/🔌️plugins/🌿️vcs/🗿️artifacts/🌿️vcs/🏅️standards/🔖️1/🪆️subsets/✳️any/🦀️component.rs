//! 🪆️ Subset root for `s.vcs.vcs@1/*` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM).
//! Exports `subset() -> SubsetDeclaration`, assembling the `🧬️schema`/`🚪️io`/`👁️viewer`/`✏️editor`/
//! `📚️examples` children — `crate::editor::vcs`/`crate::viewer::vcs` stay mounted at the plugin's
//! top-level `editor`/`viewer` modules (recipe §5 gotcha 1), not here.

use crate::artifacts::vcs::standards::v1::subsets::any::{io, schema};
use crate::artifacts::vcs::VCS_DIALECT;
use crate::editor::vcs as editor;
use crate::viewer::vcs as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;

fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::artifacts::vcs::examples::demo::source()]).as_slice()
}

fn inference_descriptors() -> &'static [::schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::vcs_artifact_inference_descriptor()]).as_slice()
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub fn subset() -> SubsetDeclaration<crate::VcsApps> {
    SubsetDeclaration {
        dialect: VCS_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::vcs_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::VcsViewer, crate::VcsApps>(viewer::create_vcs_viewer()),
        editor: editor_surface::<editor::VcsPlayApp, crate::VcsApps>(editor::create_vcs_app()),
        examples: examples(),
    }
}
