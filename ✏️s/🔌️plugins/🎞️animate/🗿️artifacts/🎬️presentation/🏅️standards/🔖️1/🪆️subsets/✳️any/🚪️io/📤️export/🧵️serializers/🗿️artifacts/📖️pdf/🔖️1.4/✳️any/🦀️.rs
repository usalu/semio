//! 🚪️ presentation -> pdf — foreign `Serializer<PresentationSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Structural first-party JSON
//! coercion between `PresentationSnapshot`'s and `PdfSnapshot`'s (unrelated) field shapes — not a real
//! presentation->pdf semantic mapping (unchanged behaviour, pre-dates this ticket) — `IoFidelity::Lossy`.

use crate::PresentationSnapshot;
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_pdf::PdfSnapshot;

pub const PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId::ANY };

pub struct PresentationIntoPdf;

impl Serializer<PresentationSnapshot> for PresentationIntoPdf {
    const INTO: Dialect = PDF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &PresentationSnapshot, _: &ArchiveChildren, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        let json = semio_framework_pack_json::to_json_string(from);
        let wire: PdfSnapshot = semio_framework_pack_json::from_json_str(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| { let mut cause=error;cause.message=format!("PresentationIntoPdf: {}",cause.message).into();IoError::from_value_error(cause) })?;
        Ok(IoOutcome::clean(IoPayload::Binary(<PdfSnapshot as store::ArtifactPack>::encode_pack(&wire))))
    }
}
