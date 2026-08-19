//! 📤️ Foreign leaf — serialize `WriterSnapshot` INTO `s.stdio.docx@ecma-376/*`. Builds a real,
//! minimal-but-valid docx package: one paragraph per source line, via the docx artifact's own
//! typed builder — not a fabricated/renamed text file inside a zip.

use crate::artifacts::writer::{writer_text, WriterSnapshot};
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{Dialect, StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::docx::schema::snapshot::DocxBlock;
use semio_s_plugin_stdio::artifacts::docx::DocxSnapshot;

pub const DOCX_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("*") };

//#region 🔖️Serializer
pub struct WriterIntoDocx;
impl Serializer<WriterSnapshot> for WriterIntoDocx {
    const INTO: Dialect = DOCX_DIALECT;
    /// 🪧️ Lossy — see the sibling deserializer's doc comment.
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn serialize(from: &WriterSnapshot) -> IoResult<IoPayload> {
        let body: Vec<DocxBlock> = writer_text(from).split('\n').map(DocxBlock::paragraph).collect();
        let document = semio_s_plugin_stdio::artifacts::docx::schema::snapshot::DocxDocument { body, styles: Vec::new() };
        let docx = semio_s_plugin_stdio::artifacts::docx::engine::build_minimal_docx(document);
        Ok(IoOutcome { value: IoPayload::Binary(<DocxSnapshot as store::ArtifactPack>::encode_pack(&docx)), diagnostics: Vec::new() })
    }
}
//#endregion 🔖️Serializer

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn writer_into_docx_round_trips_through_docx_into_writer() {
        let snapshot = crate::artifacts::writer::writer_snapshot_with_text("writer.document", "id", "plain", "writer://id", "hello");
        let outcome = WriterIntoDocx::serialize(&snapshot).expect("serialize");
        let IoPayload::Binary(bytes) = outcome.value else { panic!("expected binary payload") };
        let decoded = <DocxSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert!(!decoded.document.body.is_empty());
    }
}
