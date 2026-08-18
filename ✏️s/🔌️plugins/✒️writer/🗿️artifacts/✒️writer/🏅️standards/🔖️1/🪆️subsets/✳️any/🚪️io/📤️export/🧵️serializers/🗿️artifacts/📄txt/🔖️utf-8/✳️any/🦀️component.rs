//! 📤️ Foreign leaf — serialize `WriterSnapshot` INTO `s.stdio.txt@utf-8/*`. `s.stdio.txt` is a
//! CARRIER dialect (design.md §3): its native `Text` `IoPayload` IS the raw external file text,
//! verbatim — so this writes the document's plain content text directly, never writer's own
//! hex-encoded wire DSL (that bug — dumping the internal wire format into a plain `.txt` export —
//! is fixed here, in passing, matching the class of bug `📓️w4-sequence-report.md`'s CSV fix found).

use crate::artifacts::writer::{writer_text, WriterSnapshot};
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{Dialect, StandardId, SubsetId};

pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

//#region 🔖️Serializer
pub struct WriterIntoTxt;
impl Serializer<WriterSnapshot> for WriterIntoTxt {
    const INTO: Dialect = TXT_DIALECT;
    /// 🪧️ Lossy: only the document's content text survives — `schema`/`id`/`uri`/`language_id` have
    /// no home in a plain-text file.
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn serialize(from: &WriterSnapshot) -> IoResult<IoPayload> {
        Ok(IoOutcome { value: IoPayload::Text(writer_text(from)), diagnostics: Vec::new() })
    }
}
//#endregion 🔖️Serializer

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writer_into_txt_emits_plain_document_text() {
        let snapshot = crate::artifacts::writer::writer_snapshot_with_text("writer.document", "id", "plain", "writer://id", "hello\nworld");
        let outcome = WriterIntoTxt::serialize(&snapshot).expect("serialize");
        assert_eq!(outcome.value, IoPayload::Text("hello\nworld".into()));
    }
}
