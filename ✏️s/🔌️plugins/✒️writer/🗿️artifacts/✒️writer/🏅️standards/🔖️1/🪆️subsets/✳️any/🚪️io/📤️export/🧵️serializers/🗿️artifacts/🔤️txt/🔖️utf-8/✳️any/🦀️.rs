//! 📤️ Foreign leaf — serialize `WriterSnapshot` INTO `framework.io.text@utf-8/*`. `framework.io.text` is a
//! CARRIER dialect (design.md §3): its native `Text` `IoPayload` IS the raw external file text,
//! verbatim — so this writes the document's plain content text directly, never writer's own
//! hex-encoded wire DSL (that bug — dumping the internal wire format into a plain `.txt` export —
//! is fixed here, in passing, matching the class of bug `📓️w4-sequence-report.md`'s CSV fix found).

use crate::{writer_text, WriterSnapshot};
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use semio_framework::io_schema::{IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_artifact_reference::Dialect;

pub const TXT_DIALECT: Dialect = semio_framework::io_schema::CARRIER_TEXT;

//#region 🔖️Serializer
pub struct WriterIntoTxt;
impl Serializer<WriterSnapshot> for WriterIntoTxt {
    const INTO: Dialect = TXT_DIALECT;
    /// 🪧️ Lossy: only the document's content text survives — `schema`/`id`/`uri`/`language_id` have
    /// no home in a plain-text file.
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &WriterSnapshot, _: &ArchiveChildren, _control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        Ok(IoOutcome { value: IoPayload::Text(writer_text(from)), diagnostics: Vec::new() })
    }
}
//#endregion 🔖️Serializer

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
