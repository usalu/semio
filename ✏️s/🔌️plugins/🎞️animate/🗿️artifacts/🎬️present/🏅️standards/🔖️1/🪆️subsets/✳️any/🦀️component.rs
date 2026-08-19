//! ✳️ Subset root — `subset() -> SubsetDeclaration` (design.md §1/§2 recipe step 1). Mounts
//! `🧬️schema`/`🚪️io`/`👁️viewer`/`✏️editor`/`📚️examples` for `s.animate.present@1/*`.

use crate::artifacts::present::standards::v1::subsets::any::{io, schema};
use crate::editor::animate as editor;
use crate::viewer::animate as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;

async fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::artifacts::present::examples::demo::source()]).as_slice()
}

async fn inference_descriptors() -> &'static [::schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::present_artifact_inference_descriptor()]).as_slice()
}

pub async fn subset() -> SubsetDeclaration {
    SubsetDeclaration {
        dialect: crate::artifacts::present::ANIMATE_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::present_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::AnimatePresentViewer>(viewer::create_animate_present_viewer()),
        editor: editor_surface::<editor::AnimatePresentPlayApp>(editor::create_animate_present_app()),
        examples: examples(),
    }
}
