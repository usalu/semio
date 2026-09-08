//! 📥️ Imports the ordered page text of the PDF 1.4 base subset into a Writer document.

use crate::artifacts::writer::{writer_snapshot_with_text, WriterSnapshot, WRITER_DOCUMENT_SCHEMA};
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{Dialect, StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::pdf::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot;

pub const PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("base") };

pub struct PdfIntoWriter;
impl Deserializer<WriterSnapshot> for PdfIntoWriter {
    const FROM: Dialect = PDF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(payload: &IoPayload) -> IoResult<WriterSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "PdfIntoWriter: expected a binary payload".to_string(), diagnostics: Vec::new() });
        };
        let pdf = <PdfSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError { message: format!("PdfIntoWriter: {error}"), diagnostics: Vec::new() })?;
        let text = pdf.pages.iter().map(|page| page.text.as_str()).collect::<Vec<_>>().join("\n");
        let snapshot = writer_snapshot_with_text(WRITER_DOCUMENT_SCHEMA, "pdf-import", "plaintext", "writer://pdf-import", &text);
        Ok(IoOutcome { value: snapshot, diagnostics: Vec::new() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[semio_framework_async_macros::async_test]
    async fn pdf_page_text_vectors_match_the_json_oracle() {
        let vectors: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️page-text.json")).unwrap();
        for vector in vectors["cases"].as_array().unwrap() {
            let value = serde_json::json!({"schema":"stdio.pdf","pages":vector["pages"]});
            let pdf: PdfSnapshot = dsl::os_pack::json::from_json_str(&value.to_string()).unwrap();
            let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&pdf);
            let outcome = PdfIntoWriter::deserialize(&IoPayload::Binary(bytes)).await.expect("deserialize");
            assert_eq!(crate::artifacts::writer::writer_text(&outcome.value), vector["expected"].as_str().unwrap(), "{}", vector["name"]);
            assert_eq!(pdf.pages.iter().map(|page| page.text.as_str()).collect::<Vec<_>>().join("\n"), vector["expected"].as_str().unwrap());
        }
    }
}
