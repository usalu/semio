//! 📥️ Foreign leaf — deserialize `WriterSnapshot` FROM `s.stdio.md@commonmark/*`. The markdown
//! text becomes the writer document's plain-text content — a fresh document, never writer's own
//! hex-encoded wire DSL (the same class of bug this ticket's sequence pilot fixed for CSV; the
//! previous impl here made exactly that mistake, fixed in passing).

use crate::artifacts::writer::{writer_snapshot_with_text, WriterSnapshot, WRITER_DOCUMENT_SCHEMA};
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{Dialect, StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::md::MdSnapshot;

pub const MD_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId("*") };

//#region 🔖️Deserializer
pub struct MdIntoWriter;
impl Deserializer<WriterSnapshot> for MdIntoWriter {
    const FROM: Dialect = MD_DIALECT;
    /// 🪧️ Lossy: a markdown file carries no `schema`/`id`/`uri`/`language_id` — only content.
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn deserialize(payload: &IoPayload) -> IoResult<WriterSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError { message: "MdIntoWriter: expected a text payload".to_string(), diagnostics: Vec::new() });
        };
        let md = <MdSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| IoError { message: format!("MdIntoWriter: {error}"), diagnostics: Vec::new() })?;
        let snapshot = writer_snapshot_with_text(WRITER_DOCUMENT_SCHEMA, "md-import", "plain", "writer://md-import", &md.to_text());
        Ok(IoOutcome { value: snapshot, diagnostics: Vec::new() })
    }
}
//#endregion 🔖️Deserializer

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn md_into_writer_uses_the_markdown_text_as_document_text() {
        let outcome = MdIntoWriter::deserialize(&IoPayload::Text("hello world".into())).expect("deserialize");
        assert_eq!(crate::artifacts::writer::writer_text(&outcome.value), "hello world");
        assert_eq!(outcome.value.language_id, "plain");
    }
}
