//! ✳️ BIM subset `any` root: mounts `schema`, `io`, `viewer`, `editor` and `examples` and exports the one `subset() -> SubsetDeclaration` of this artifact.
//! The io surface is [`io::io`]: the native codec plus the IFC, glTF and SVG hops, so the host registers them with the subset.

use crate::editor::bim as editor;
use semio_s_artifact_bim_model_io::standards::v1::subsets::any::io;
use semio_s_artifact_bim_model_viewer::viewer::bim as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;

//#region 🔖️Examples
/// 📚️ The example documents of the subset: the demo, the house and the office.
pub fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![semio_s_artifact_bim_model_formula::examples::demo::source(), semio_s_artifact_bim_model_examples::examples::house::source(), semio_s_artifact_bim_model_formula::examples::office::source()]).as_slice()
}
//#endregion 🔖️Examples

//#region 🔖️Inferences
fn inference_descriptors() -> &'static [::semio_framework_schema_registry::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::semio_framework_schema_registry::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::bim_model_inference_descriptor()]).as_slice()
}
//#endregion 🔖️Inferences

//#region 🔖️Subset
/// 🌳️ The complete declaration of subset `any`.
pub fn subset<A: crate::BimApplication>() -> SubsetDeclaration<A> {
    SubsetDeclaration {
        dialect: semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::BIM_MODEL_DIALECT,
        schema: SchemaDeclaration { descriptor: semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::bim_model_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: semio_s_artifact_bim_model_io::standards::v1::subsets::any::io::io(),
        viewer: viewer_surface::<semio_s_artifact_bim_model_viewer::viewer::bim::BimModelViewer, A>(semio_s_artifact_bim_model_viewer::viewer::bim::create_bim_viewer()),
        editor: editor_surface::<semio_s_artifact_bim_model_bim::editor::bim::BimModelApp, A>(semio_s_artifact_bim_model_bim::editor::bim::create_bim_app()),
        examples: examples(),
    }
}
//#endregion 🔖️Subset
