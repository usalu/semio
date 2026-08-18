//! 📤️ Foreign leaf — serialize `WriterSnapshot` INTO `s.stdio.pdf@1.4/*` (the frozen 1.4 `PageDoc`
//! subset — see stdio's own `📄️pdf` artifact-root doc comment: TWO independent `PdfSnapshot` types
//! share the `stdio.pdf` id family, 1.4's is this plain single-page `PageDoc` shape, never the
//! canonical 1.7 object-model one).

use crate::artifacts::writer::{writer_text, WriterSnapshot};
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{Dialect, StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::{PageDoc, PdfSnapshot};
use semio_s_plugin_stdio::artifacts::pdf::STDIO_PDF_DOCUMENT_SCHEMA;

pub const PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("*") };

//#region 🔖️Serializer
pub struct WriterIntoPdf;
impl Serializer<WriterSnapshot> for WriterIntoPdf {
    const INTO: Dialect = PDF_DIALECT;
    /// 🪧️ Lossy — see the sibling deserializer's doc comment.
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn serialize(from: &WriterSnapshot) -> IoResult<IoPayload> {
        let pdf = PdfSnapshot { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), page: PageDoc { width: 612.0, height: 792.0, text: writer_text(from) } };
        Ok(IoOutcome { value: IoPayload::Binary(<PdfSnapshot as store::ArtifactPack>::encode_pack(&pdf)), diagnostics: Vec::new() })
    }
}
//#endregion 🔖️Serializer

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writer_into_pdf_round_trips_through_pdf_into_writer() {
        let snapshot = crate::artifacts::writer::writer_snapshot_with_text("writer.document", "id", "plain", "writer://id", "hello");
        let outcome = WriterIntoPdf::serialize(&snapshot).expect("serialize");
        let IoPayload::Binary(bytes) = outcome.value else { panic!("expected binary payload") };
        let decoded = <PdfSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded.page.text, "hello");
    }
}
