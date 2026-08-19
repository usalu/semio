//! 📥️ Foreign leaf — deserialize `WriterSnapshot` FROM `s.stdio.docx@ecma-376/*`. Docx's native
//! `Binary` `IoPayload` is its own pack (`DocxSnapshot::decode_pack`); each paragraph's runs are
//! concatenated, paragraphs joined by newlines — the honest text projection of the typed docx
//! document (non-`Paragraph` blocks, e.g. tables, are honestly skipped rather than fabricating
//! text).

use crate::artifacts::writer::{writer_snapshot_with_text, WriterSnapshot, WRITER_DOCUMENT_SCHEMA};
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{Dialect, StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::docx::schema::snapshot::DocxBlock;
use semio_s_plugin_stdio::artifacts::docx::DocxSnapshot;

pub const DOCX_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("*") };

//#region 🔖️Deserializer
pub struct DocxIntoWriter;
impl Deserializer<WriterSnapshot> for DocxIntoWriter {
    const FROM: Dialect = DOCX_DIALECT;
    /// 🪧️ Lossy: only paragraph run text survives — `schema`/`id`/`uri`/`language_id` have no home
    /// in a docx package, tables are dropped, and run-level formatting is not modeled.
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(payload: &IoPayload) -> IoResult<WriterSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "DocxIntoWriter: expected a binary payload".to_string(), diagnostics: Vec::new() });
        };
        let docx = <DocxSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError { message: format!("DocxIntoWriter: {error}"), diagnostics: Vec::new() })?;
        let text = docx
            .document
            .body
            .iter()
            .filter_map(|block| match block {
                DocxBlock::Paragraph(p) => Some(p.runs.iter().map(|r| r.text.as_str()).collect::<String>()),
                DocxBlock::Table(_) => None,
            })
            .collect::<Vec<_>>()
            .join("\n");
        let snapshot = writer_snapshot_with_text(WRITER_DOCUMENT_SCHEMA, "docx-import", "plain", "writer://docx-import", &text);
        Ok(IoOutcome { value: snapshot, diagnostics: Vec::new() })
    }
}
//#endregion 🔖️Deserializer

#[cfg(test)]
mod tests {
    use super::*;
    use semio_s_plugin_stdio::artifacts::docx::engine::build_minimal_docx;
    use semio_s_plugin_stdio::artifacts::docx::schema::snapshot::DocxDocument;

    #[test]
    async fn docx_into_writer_joins_paragraph_runs() {
        let body: Vec<DocxBlock> = "line one\nline two".split('\n').map(DocxBlock::paragraph).collect();
        let docx = build_minimal_docx(DocxDocument { body, styles: Vec::new() });
        let bytes = <DocxSnapshot as store::ArtifactPack>::encode_pack(&docx);
        let outcome = DocxIntoWriter::deserialize(&IoPayload::Binary(bytes)).expect("deserialize");
        assert_eq!(crate::artifacts::writer::writer_text(&outcome.value), "line one\nline two");
    }
}
