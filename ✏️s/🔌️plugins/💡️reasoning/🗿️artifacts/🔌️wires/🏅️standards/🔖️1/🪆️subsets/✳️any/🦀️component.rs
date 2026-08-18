//! 🪆️ Wires subset `1/*` root — `pub fn subset() -> SubsetDeclaration` assembling
//! `schema`/`io`/`viewer`/`editor`/`examples` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §1/§2).

use crate::artifacts::wires::standards::v1::subsets::any::{io, schema};
use crate::editor::wires as editor;
use crate::viewer::wires as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::{Dialect, ExampleSource};
use std::sync::OnceLock;

//#region 🔖️Dialect
pub const DIALECT: Dialect = crate::artifacts::wires::WIRES_DIALECT;
//#endregion 🔖️Dialect

//#region 🔖️Examples
fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::artifacts::wires::examples::demo::source()]).as_slice()
}
//#endregion 🔖️Examples

//#region 🔖️Inferences
fn inference_descriptors() -> &'static [::schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::wires_artifact_inference_descriptor()]).as_slice()
}
//#endregion 🔖️Inferences

//#region 🔖️Subset
pub fn subset() -> SubsetDeclaration {
    SubsetDeclaration {
        dialect: DIALECT,
        schema: SchemaDeclaration { descriptor: schema::wires_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::WiresViewer>(viewer::create_wires_viewer()),
        editor: editor_surface::<editor::ReasoningWiresPlayApp>(editor::create_wires_app()),
        examples: examples(),
    }
}
//#endregion 🔖️Subset
