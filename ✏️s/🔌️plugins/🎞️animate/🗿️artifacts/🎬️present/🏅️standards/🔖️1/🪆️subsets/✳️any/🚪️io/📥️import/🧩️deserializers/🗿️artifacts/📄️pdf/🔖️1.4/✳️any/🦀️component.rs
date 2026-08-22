//! 🚪️ present <- pdf — foreign `Deserializer<PresentSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Structural `serde_json`
//! coercion between `PdfSnapshot`'s and `PresentSnapshot`'s (unrelated) field shapes — not a real
//! pdf->present semantic mapping (unchanged behaviour, pre-dates this ticket) — `IoFidelity::Lossy`.

use crate::artifacts::present::PresentSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::pdf::PdfSnapshot;

pub const PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId::ANY };

pub struct PdfIntoPresent;

impl Deserializer<PresentSnapshot> for PdfIntoPresent {
    const FROM: Dialect = PDF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(payload: &IoPayload) -> IoResult<PresentSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "PdfIntoPresent: expected a binary pdf payload".to_string(), diagnostics: Vec::new() });
        };
        let wire = <PdfSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError { message: format!("PdfIntoPresent: {error}"), diagnostics: Vec::new() })?;
        let value = serde_json::to_value(&wire).map_err(|error| IoError { message: format!("PdfIntoPresent: {error}"), diagnostics: Vec::new() })?;
        let snapshot: PresentSnapshot = serde_json::from_value(value).map_err(|error| IoError { message: format!("PdfIntoPresent: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(snapshot).await)
    }
}
