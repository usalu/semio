//! 🚪️ presentation <- pdf — foreign `Deserializer<PresentationSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Structural first-party JSON
//! coercion between `PdfSnapshot`'s and `PresentationSnapshot`'s (unrelated) field shapes — not a real
//! pdf->presentation semantic mapping (unchanged behaviour, pre-dates this ticket) — `IoFidelity::Lossy`.

use crate::artifacts::presentation::PresentationSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::pdf::PdfSnapshot;

pub const PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId::ANY };

pub struct PdfIntoPresentation;

impl Deserializer<PresentationSnapshot> for PdfIntoPresentation {
    const FROM: Dialect = PDF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(payload: &IoPayload) -> IoResult<PresentationSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "PdfIntoPresentation: expected a binary pdf payload".to_string(), diagnostics: Vec::new() });
        };
        let wire = <PdfSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError { message: format!("PdfIntoPresentation: {error}"), diagnostics: Vec::new() })?;
        let json = dsl::json::to_json_string(&wire);
        let snapshot: PresentationSnapshot = dsl::json::from_json_str(&json).map_err(|error| IoError { message: format!("PdfIntoPresentation: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(snapshot))
    }
}
