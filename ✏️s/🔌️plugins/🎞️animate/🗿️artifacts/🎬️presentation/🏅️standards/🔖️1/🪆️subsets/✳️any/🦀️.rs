//! ✳️ Subset root — `subset() -> SubsetDeclaration` (design.md §1/§2 recipe step 1). Mounts
//! `🧬️schema`/`🚪️io`/`👁️viewer`/`✏️editor`/`📚️examples` for `s.animate.presentation@1/*`.

use crate::artifacts::presentation::standards::v1::subsets::any::{io, schema};
use crate::editor::animate as editor;
use crate::viewer::animate as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;

fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::artifacts::presentation::examples::demo::source()]).as_slice()
}

fn inference_descriptors() -> &'static [::schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::presentation_artifact_inference_descriptor()]).as_slice()
}

pub fn subset<PA>() -> SubsetDeclaration<PA>
where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<editor::AnimatePresentationPlayApp>>>
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<viewer::AnimatePresentationViewer>>>,
{
    SubsetDeclaration {
        dialect: crate::artifacts::presentation::ANIMATE_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::presentation_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::AnimatePresentationViewer, PA>(viewer::create_animate_presentation_viewer()),
        editor: editor_surface::<editor::AnimatePresentationPlayApp, PA>(editor::create_animate_presentation_app()),
        examples: examples(),
    }
}
