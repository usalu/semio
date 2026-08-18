//! 📥️ Foreign leaf — deserialize `WriterSnapshot` FROM `s.stdio.pdf@1.4/*` (the frozen 1.4
//! `PageDoc` subset — see the sibling export serializer's doc comment). PDF's native `Binary`
//! `IoPayload` is its own pack (`PdfSnapshot::decode_pack`); the page's extracted text becomes the
//! writer document's plain-text content.

use crate::artifacts::writer::{writer_snapshot_with_text, WriterSnapshot, WRITER_DOCUMENT_SCHEMA};
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{Dialect, StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot;

pub const PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("*") };

//#region 🔖️Deserializer
pub struct PdfIntoWriter;
impl Deserializer<WriterSnapshot> for PdfIntoWriter {
    const FROM: Dialect = PDF_DIALECT;
    /// 🪧️ Lossy: only the frozen 1.4 `PageDoc`'s extracted page text survives —
    /// `schema`/`id`/`uri`/`language_id` have no home in a pdf page, and page layout/fonts/images
    /// are dropped entirely by the (documented, pre-existing) frozen-stub `PageDoc` model itself.
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn deserialize(payload: &IoPayload) -> IoResult<WriterSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "PdfIntoWriter: expected a binary payload".to_string(), diagnostics: Vec::new() });
        };
        let pdf = <PdfSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError { message: format!("PdfIntoWriter: {error}"), diagnostics: Vec::new() })?;
        let snapshot = writer_snapshot_with_text(WRITER_DOCUMENT_SCHEMA, "pdf-import", "plain", "writer://pdf-import", &pdf.page.text);
        Ok(IoOutcome { value: snapshot, diagnostics: Vec::new() })
    }
}
//#endregion 🔖️Deserializer

#[cfg(test)]
mod tests {
    use super::*;
    use semio_s_plugin_stdio::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PageDoc;

    #[test]
    fn pdf_into_writer_extracts_page_text() {
        let pdf = PdfSnapshot { schema: "s.stdio.pdf".into(), page: PageDoc { width: 612.0, height: 792.0, text: "hello".into() } };
        let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&pdf);
        let outcome = PdfIntoWriter::deserialize(&IoPayload::Binary(bytes)).expect("deserialize");
        assert_eq!(crate::artifacts::writer::writer_text(&outcome.value), "hello");
    }
}
