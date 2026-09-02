//! 🚪️ presentation -> pdf — foreign `Serializer<PresentationSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Structural first-party JSON
//! coercion between `PresentationSnapshot`'s and `PdfSnapshot`'s (unrelated) field shapes — not a real
//! presentation->pdf semantic mapping (unchanged behaviour, pre-dates this ticket) — `IoFidelity::Lossy`.

use crate::artifacts::presentation::PresentationSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::pdf::PdfSnapshot;

pub const PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId::ANY };

pub struct PresentationIntoPdf;

impl Serializer<PresentationSnapshot> for PresentationIntoPdf {
    const INTO: Dialect = PDF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &PresentationSnapshot) -> IoResult<IoPayload> {
        let json = dsl::json::to_json_string(from);
        let wire: PdfSnapshot = dsl::json::from_json_str(&json).map_err(|error| IoError { message: format!("PresentationIntoPdf: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(<PdfSnapshot as store::ArtifactPack>::encode_pack(&wire))))
    }
}
