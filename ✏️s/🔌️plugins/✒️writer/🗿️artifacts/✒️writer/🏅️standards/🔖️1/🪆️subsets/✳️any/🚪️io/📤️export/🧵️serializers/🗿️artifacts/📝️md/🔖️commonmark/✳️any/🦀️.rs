//! 📤️ Foreign leaf — serialize `WriterSnapshot` INTO `s.stdio.md@commonmark/*`. `s.stdio.md` is NOT
//! a carrier dialect (design.md §3 names only `s.stdio.binary@raw`/`s.stdio.txt@utf-8`), so this
//! hop's `IoPayload` is `MdSnapshot`'s own genuine wire DSL (envelope-wrapped `print_dsl`), not raw
//! reader-facing markdown — a real `.md` file save routes `writer → txt` (the carrier), not through
//! this hop directly. The document's plain content text becomes a single markdown paragraph, never
//! writer's own hex-encoded wire DSL (dumping the internal wire format into this hop is the same
//! class of bug this ticket's sequence pilot's CSV fix found; fixed here in passing).

use crate::artifacts::writer::{writer_text, WriterSnapshot};
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{Dialect, StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::md::MdSnapshot;

pub const MD_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId("*") };

//#region 🔖️Serializer
pub struct WriterIntoMd;
impl Serializer<WriterSnapshot> for WriterIntoMd {
    const INTO: Dialect = MD_DIALECT;
    /// 🪧️ Lossy: only the document's content text survives — `schema`/`id`/`uri`/`language_id` have
    /// no home in a markdown file.
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn serialize(from: &WriterSnapshot) -> IoResult<IoPayload> {
        let md = MdSnapshot::from_text(&writer_text(from));
        Ok(IoOutcome { value: IoPayload::Text(store::ArtifactDsl::print_dsl(&md)), diagnostics: Vec::new() })
    }
}
//#endregion 🔖️Serializer

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn writer_into_md_carries_the_document_text() {
        let snapshot = crate::artifacts::writer::writer_snapshot_with_text("writer.document", "id", "plain", "writer://id", "hello world");
        let outcome = WriterIntoMd::serialize(&snapshot).expect("serialize");
        let IoPayload::Text(text) = outcome.value else { panic!("expected text payload") };
        assert!(text.contains("hello world"));
    }
}
