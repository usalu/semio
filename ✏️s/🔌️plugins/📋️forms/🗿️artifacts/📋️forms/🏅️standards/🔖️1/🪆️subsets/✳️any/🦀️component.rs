//! ✳️ Forms subset `any` root (design.md §1) — `pub fn subset() -> SubsetDeclaration`, assembling
//! schema/io/viewer/editor/examples. New file: this level did not exist before ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM. `editor`/`viewer` are reached via the
//! TOP-LEVEL `crate::editor::forms`/`crate::viewer::forms` modules (`📦️glue.rs`'s pre-existing
//! per-plugin convention — `grep -n "pub mod editor\b\|pub mod viewer\b" 📦️glue.rs` confirms — not
//! nested under `artifacts::forms::…`, per `📓️recipe-subset.md` §5 gotcha 1).

pub async fn subset() -> semio_framework_plugin::app::declarations::SubsetDeclaration {
    use crate::artifacts::forms::standards::v1::subsets::any::{io, schema};
    use crate::artifacts::forms::FORMS_DIALECT;
    use crate::editor::forms as editor;
    use crate::viewer::forms as viewer;
    use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
    use semio_framework_plugin::ExampleSource;
    use std::sync::OnceLock;

    async fn examples() -> &'static [ExampleSource] {
        static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
        EXAMPLES.get_or_init(|| vec![crate::artifacts::forms::examples::demo::source()]).as_slice()
    }

    async fn inference_descriptors() -> &'static [::schema::ArtifactInferenceDescriptor] {
        static DESCRIPTORS: OnceLock<Vec<::schema::ArtifactInferenceDescriptor>> = OnceLock::new();
        DESCRIPTORS.get_or_init(|| vec![schema::inferences::forms_artifact_inference_descriptor()]).as_slice()
    }

    SubsetDeclaration {
        dialect: FORMS_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::forms_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::FormsViewer>(viewer::create_forms_viewer()),
        editor: editor_surface::<editor::FormsPlayApp>(editor::create_forms_app()),
        examples: examples(),
    }
}
