//! ✳️ Writer subset `any` root — mounts `schema`/`io`/`viewer`/`editor`/`examples` and exports the
//! one `subset() -> SubsetDeclaration` this owner is responsible for (design.md §1).

use crate::artifacts::writer::standards::v1::subsets::any::{io, schema};
use crate::editor::writer as editor;
use crate::viewer::writer as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;

//#region 🔖️Examples
fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::artifacts::writer::examples::demo::source()]).as_slice()
}
//#endregion 🔖️Examples

//#region 🔖️Inferences
fn inference_descriptors() -> &'static [::schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::writer_artifact_inference_descriptor()]).as_slice()
}
//#endregion 🔖️Inferences

//#region 🔖️Subset
pub fn subset() -> SubsetDeclaration {
    SubsetDeclaration {
        dialect: crate::artifacts::writer::WRITER_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::writer_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::WriterViewer>(viewer::create_writer_viewer()),
        editor: editor_surface::<editor::WriterPlayApp>(editor::create_writer_app()),
        examples: examples(),
    }
}
//#endregion 🔖️Subset

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subset_dialect_is_the_canonical_writer_dialect() {
        assert_eq!(subset().dialect, crate::artifacts::writer::WRITER_DIALECT);
    }

    #[test]
    fn subset_declares_ten_io_entries() {
        assert_eq!(subset().io.entries.len(), 10);
    }
}
