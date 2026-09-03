//! 🪆️ Subset root for `s.mathematical.equation@1/*` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-
//! SUBSET-MECHANISM). Exports `subset() -> SubsetDeclaration`, assembling the `🧬️schema`/`🚪️io`/
//! `👁️viewer`/`✏️editor`/`📚️examples` children — `crate::editor::equation`/
//! `crate::viewer::equation` stay mounted at the plugin's top-level `editor`/`viewer` modules
//! (recipe §5 gotcha 1), not here.

use crate::artifacts::equation::standards::v1::subsets::any::{io, schema};
use crate::artifacts::equation::EQUATION_DIALECT;
use crate::editor::equation as editor;
use crate::viewer::equation as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;

async fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::artifacts::equation::examples::demo::source()]).as_slice()
}

async fn inference_descriptors() -> &'static [::schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::equation_artifact_inference_descriptor()]).as_slice()
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub async fn subset() -> SubsetDeclaration {
    SubsetDeclaration {
        dialect: EQUATION_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::equation_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::EquationViewer>(viewer::create_equation_viewer()),
        editor: editor_surface::<editor::EquationPlayApp>(editor::create_equation_app()),
        examples: examples(),
    }
}
