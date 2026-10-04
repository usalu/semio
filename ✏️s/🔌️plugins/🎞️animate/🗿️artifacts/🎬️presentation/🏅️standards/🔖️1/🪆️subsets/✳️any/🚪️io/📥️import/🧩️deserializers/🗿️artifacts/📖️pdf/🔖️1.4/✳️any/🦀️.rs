//! 🚪️ presentation <- pdf — foreign `Deserializer<PresentationSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Structural first-party JSON
//! coercion between `PdfSnapshot`'s and `PresentationSnapshot`'s (unrelated) field shapes — not a real
//! pdf->presentation semantic mapping (unchanged behaviour, pre-dates this ticket) — `IoFidelity::Lossy`.

use crate::PresentationSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_pdf::PdfSnapshot;

pub const PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId::ANY };

pub struct PdfIntoPresentation;

impl Deserializer<PresentationSnapshot> for PdfIntoPresentation {
    const FROM: Dialect = PDF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(payload: &IoPayload) -> IoResult<PresentationSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"PdfIntoPresentation: expected a binary pdf payload")));
        };
        let wire = <PdfSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError::from_value_error(match error.into_value_error() { Ok(cause) => semio_framework_value::ValueError::new(cause.kind, format!("PdfIntoPresentation: {}", cause.message)), Err(error) => semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, format!("PdfIntoPresentation: in-memory decode reported a transport failure: {error}")) }))?;
        let json = semio_framework_pack_json::to_json_string(&wire);
        let snapshot: PresentationSnapshot = semio_framework_pack_json::from_json_str(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| { let mut cause=error;cause.message=format!("PdfIntoPresentation: {}",cause.message);IoError::from_value_error(cause) })?;
        Ok(IoOutcome::clean(snapshot))
    }
}
