//! 🪆️ Subset root for `s.mathematical.equation@1/*` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-
//! SUBSET-MECHANISM). Exports `subset() -> SubsetDeclaration`, assembling the `🧬️schema`/`🚪️io`/
//! `👁️viewer`/`✏️editor`/`📚️examples` children — `crate::editor::equation`/
//! `crate::viewer::equation` stay mounted at the plugin's top-level `editor`/`viewer` modules
//! (recipe §5 gotcha 1), not here.

use crate::editor::equation as editor;
use crate::standards::v1::subsets::any::{io, schema};
use crate::viewer::equation as viewer;
use crate::EQUATION_DIALECT;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;

fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::examples::demo::source()]).as_slice()
}

fn inference_descriptors() -> &'static [::framework_schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::framework_schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::equation_artifact_inference_descriptor()]).as_slice()
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub fn subset<A: crate::EquationApplication>() -> SubsetDeclaration<A> {
    SubsetDeclaration {
        dialect: EQUATION_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::equation_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::EquationViewer, A>(viewer::create_equation_viewer()),
        editor: editor_surface::<editor::EquationPlayApp, A>(editor::create_equation_app()),
        examples: examples(),
    }
}
