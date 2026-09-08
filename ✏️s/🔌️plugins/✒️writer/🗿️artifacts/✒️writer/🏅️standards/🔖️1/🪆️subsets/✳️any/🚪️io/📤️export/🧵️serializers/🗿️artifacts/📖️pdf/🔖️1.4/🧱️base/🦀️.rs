//! 📤️ Exports Writer text as one US Letter page in the PDF 1.4 base subset.

use crate::{writer_text, WriterSnapshot};
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::schema::snapshot::{PageDoc, PdfSnapshot};
use semio_s_artifact_stdio_pdf::STDIO_PDF_DOCUMENT_SCHEMA;

pub const PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("base") };

pub struct WriterIntoPdf;
impl Serializer<WriterSnapshot> for WriterIntoPdf {
    const INTO: Dialect = PDF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &WriterSnapshot) -> IoResult<IoPayload> {
        let pdf = PdfSnapshot { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), pages: vec![PageDoc { width: PageDoc::DEFAULT_WIDTH, height: PageDoc::DEFAULT_HEIGHT, text: writer_text(from) }] };
        Ok(IoOutcome { value: IoPayload::Binary(<PdfSnapshot as store::ArtifactPack>::encode_pack(&pdf)), diagnostics: Vec::new() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[semio_framework_async_macros::async_test]
    async fn writer_into_pdf_preserves_text_and_page_size() {
        let snapshot = crate::writer_snapshot_with_text("writer.document", "id", "plaintext", "writer://id", "hello");
        let outcome = WriterIntoPdf::serialize(&snapshot).await.expect("serialize");
        let IoPayload::Binary(bytes) = outcome.value else { panic!("expected binary payload") };
        let decoded = <PdfSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        let actual: serde_json::Value = serde_json::from_str(&dsl::os_pack::json::to_json_string(&decoded)).unwrap();
        assert_eq!(actual["pages"], serde_json::json!([{"width":612.0,"height":792.0,"text":"hello"}]));
    }
}
