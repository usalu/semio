//! 📥️ Foreign leaf — deserialize `WriterSnapshot` FROM `s.stdio.txt@utf-8/*`. `s.stdio.txt` is a
//! CARRIER dialect (design.md §3): its native `Text` `IoPayload` IS the raw external file text,
//! verbatim, so `TxtSnapshot::parse_dsl` is an identity wrap over that raw text. The whole raw body
//! becomes the writer document's plain-text content — a fresh document, never writer's own
//! hex-encoded wire DSL (that bug — treating arbitrary prose as writer's internal format — is fixed
//! here, in passing, matching the class of bug `📓️w4-sequence-report.md`'s CSV fix found).

use crate::artifacts::writer::{writer_snapshot_with_text, WriterSnapshot, WRITER_DOCUMENT_SCHEMA};
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{Dialect, StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::txt::TxtSnapshot;

pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

//#region 🔖️Deserializer
pub struct TxtIntoWriter;
impl Deserializer<WriterSnapshot> for TxtIntoWriter {
    const FROM: Dialect = TXT_DIALECT;
    /// 🪧️ Lossy: a plain-text file carries no `schema`/`id`/`uri`/`language_id` — only content. The
    /// writer document it seeds is a fresh one (`"txt-import"`), never a restoration of a prior
    /// writer document's full identity.
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn deserialize(payload: &IoPayload) -> IoResult<WriterSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError { message: "TxtIntoWriter: expected a text payload".to_string(), diagnostics: Vec::new() });
        };
        let txt = <TxtSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| IoError { message: format!("TxtIntoWriter: {error}"), diagnostics: Vec::new() })?;
        let snapshot = writer_snapshot_with_text(WRITER_DOCUMENT_SCHEMA, "txt-import", "plain", "writer://txt-import", &txt.to_body());
        Ok(IoOutcome { value: snapshot, diagnostics: Vec::new() })
    }
}
//#endregion 🔖️Deserializer

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn txt_into_writer_uses_the_raw_body_as_document_text() {
        let outcome = TxtIntoWriter::deserialize(&IoPayload::Text("hello\nworld".into())).expect("deserialize");
        assert_eq!(crate::artifacts::writer::writer_text(&outcome.value), "hello\nworld");
        assert_eq!(outcome.value.language_id, "plain");
    }
}
