//! 🪆️ Subset root for `s.mathematical.mathematical@1/*` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-
//! SUBSET-MECHANISM). Exports `subset() -> SubsetDeclaration`, assembling the `🧬️schema`/`🚪️io`/
//! `👁️viewer`/`✏️editor`/`📚️examples` children — `crate::editor::mathematical`/
//! `crate::viewer::mathematical` stay mounted at the plugin's top-level `editor`/`viewer` modules
//! (recipe §5 gotcha 1), not here.

use crate::artifacts::mathematical::standards::v1::subsets::any::{io, schema};
use crate::artifacts::mathematical::MATHEMATICAL_DIALECT;
use crate::editor::mathematical as editor;
use crate::viewer::mathematical as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;

async fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::artifacts::mathematical::examples::demo::source()]).as_slice()
}

async fn inference_descriptors() -> &'static [::schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::mathematical_artifact_inference_descriptor()]).as_slice()
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub async fn subset() -> SubsetDeclaration {
    SubsetDeclaration {
        dialect: MATHEMATICAL_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::mathematical_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::MathematicalViewer>(viewer::create_mathematical_viewer()),
        editor: editor_surface::<editor::MathematicalPlayApp>(editor::create_mathematical_app()),
        examples: examples(),
    }
}
