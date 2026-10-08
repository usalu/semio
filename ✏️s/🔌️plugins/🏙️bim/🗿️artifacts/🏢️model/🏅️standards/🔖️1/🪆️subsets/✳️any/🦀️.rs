//! ✳️ BIM subset `any` root: mounts `schema`, `io`, `viewer`, `editor` and `examples` and exports the one `subset() -> SubsetDeclaration` of this artifact.
//! The io surface is [`io::io`]: the native codec plus the IFC, glTF and SVG hops, so the host registers them with the subset.

use crate::editor::bim as editor;
use crate::standards::v1::subsets::any::{io, schema};
use crate::viewer::bim as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;

//#region 🔖️Examples
/// 📚️ The example documents of the subset: the demo, the house and the office.
pub fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::examples::demo::source(), crate::examples::house::source(), crate::examples::office::source()]).as_slice()
}
//#endregion 🔖️Examples

//#region 🔖️Inferences
fn inference_descriptors() -> &'static [::semio_framework_schema_registry::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::semio_framework_schema_registry::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::bim_model_inference_descriptor()]).as_slice()
}
//#endregion 🔖️Inferences

//#region 🔖️Subset
/// 🌳️ The complete declaration of subset `any`.
pub fn subset<A: crate::BimApplication>() -> SubsetDeclaration<A> {
    SubsetDeclaration {
        dialect: crate::BIM_MODEL_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::bim_model_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::BimModelViewer, A>(viewer::create_bim_viewer()),
        editor: editor_surface::<editor::BimModelApp, A>(editor::create_bim_app()),
        examples: examples(),
    }
}
//#endregion 🔖️Subset
