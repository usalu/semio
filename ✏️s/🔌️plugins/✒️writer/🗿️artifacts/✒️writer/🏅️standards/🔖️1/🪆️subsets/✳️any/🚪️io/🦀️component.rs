//! 🚪️ IO s.writer (1/✳️any) — the declaration tree owns this table (design.md §2/§3). Every foreign
//! leaf below is a real `Serializer<WriterSnapshot>`/`Deserializer<WriterSnapshot>` registered via
//! `serializer_entry`/`deserializer_entry`; the native codec lives unsplit under `📸️snapshot/`,
//! `🔺️diff/`, `🧬️mutations/`, `💡️inferences/` (design.md §1 CORRECTION).

use crate::artifacts::writer::{WriterMutation, WriterSnapshot, WRITER_DIALECT, WRITER_DOCUMENT_SCHEMA};
use semio_framework::io::io_mechanism::{deserializer_entry, serializer_entry, IoEntry};
use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};

//#region 🔖️Io
pub fn io() -> IoDeclaration {
    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: None, binary: None },
            diff: LanguagePair { text: None, binary: None },
            mutations: LanguagePair { text: None, binary: None },
            inferences: None,
            codec: store::ArtifactCodec::of::<WriterSnapshot, WriterMutation>(WRITER_DOCUMENT_SCHEMA.to_string()),
        },
        entries: entries(),
    }
}

fn entries() -> &'static [IoEntry] {
    use crate::artifacts::writer::io::export::serializers::artifacts as export;
    use crate::artifacts::writer::io::import::deserializers::artifacts as import;
    static ENTRIES: std::sync::OnceLock<Vec<IoEntry>> = std::sync::OnceLock::new();
    ENTRIES
        .get_or_init(|| {
            vec![
                deserializer_entry::<WriterSnapshot, import::txt::v_utf_8::any::TxtIntoWriter>(WRITER_DIALECT),
                serializer_entry::<WriterSnapshot, export::txt::v_utf_8::any::WriterIntoTxt>(WRITER_DIALECT),
                deserializer_entry::<WriterSnapshot, import::json::v_rfc8259::any::JsonIntoWriter>(WRITER_DIALECT),
                serializer_entry::<WriterSnapshot, export::json::v_rfc8259::any::WriterIntoJson>(WRITER_DIALECT),
                deserializer_entry::<WriterSnapshot, import::md::v_commonmark::any::MdIntoWriter>(WRITER_DIALECT),
                serializer_entry::<WriterSnapshot, export::md::v_commonmark::any::WriterIntoMd>(WRITER_DIALECT),
                deserializer_entry::<WriterSnapshot, import::pdf::v1_4::any::PdfIntoWriter>(WRITER_DIALECT),
                serializer_entry::<WriterSnapshot, export::pdf::v1_4::any::WriterIntoPdf>(WRITER_DIALECT),
                deserializer_entry::<WriterSnapshot, import::docx::v_ecma_376::any::DocxIntoWriter>(WRITER_DIALECT),
                serializer_entry::<WriterSnapshot, export::docx::v_ecma_376::any::WriterIntoDocx>(WRITER_DIALECT),
            ]
        })
        .as_slice()
}
//#endregion 🔖️Io

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn io_declares_ten_entries_five_formats_both_directions() {
        assert_eq!(entries().len(), 10);
    }
}
//#endregion 🧪️Tests
