//! 📤️ Foreign leaf — serialize `WriterSnapshot` INTO `s.stdio.docx@ecma-376/*`. Builds a real,
//! minimal-but-valid docx package: one paragraph per source line, via the docx artifact's own
//! typed builder — not a fabricated/renamed text file inside a zip.

use crate::{writer_text, WriterSnapshot};
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use semio_framework::io_schema::{IoFidelity, IoOutcome, IoPayload, IoResult};
use {semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_docx::schema::snapshot::DocxBlock;
use semio_s_artifact_stdio_docx::DocxSnapshot;

pub const DOCX_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("*") };

//#region 🔖️Serializer
pub struct WriterIntoDocx;
impl Serializer<WriterSnapshot> for WriterIntoDocx {
    const INTO: Dialect = DOCX_DIALECT;
    /// 🪧️ Lossy — see the sibling deserializer's doc comment.
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &WriterSnapshot, _: &ArchiveChildren, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        let body: Vec<DocxBlock> = writer_text(from).split('\n').map(DocxBlock::paragraph).collect();
        let document = semio_s_artifact_stdio_docx::schema::snapshot::DocxDocument { body, styles: Vec::new() };
        let docx = semio_s_artifact_stdio_docx::schema::construction::build_minimal_docx(document);
        Ok(IoOutcome { value: IoPayload::Binary(<DocxSnapshot as store::ArtifactPack>::encode_pack(&docx)), diagnostics: Vec::new() })
    }
}
//#endregion 🔖️Serializer

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
