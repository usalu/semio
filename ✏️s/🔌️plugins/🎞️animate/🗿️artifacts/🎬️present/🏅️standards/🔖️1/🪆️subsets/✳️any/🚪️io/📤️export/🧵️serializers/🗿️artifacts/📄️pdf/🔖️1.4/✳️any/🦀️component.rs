//! 🚪️ present -> pdf — foreign `Serializer<PresentSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Structural `serde_json`
//! coercion between `PresentSnapshot`'s and `PdfSnapshot`'s (unrelated) field shapes — not a real
//! present->pdf semantic mapping (unchanged behaviour, pre-dates this ticket) — `IoFidelity::Lossy`.

use crate::artifacts::present::PresentSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::pdf::PdfSnapshot;

pub const PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId::ANY };

pub struct PresentIntoPdf;

impl Serializer<PresentSnapshot> for PresentIntoPdf {
    const INTO: Dialect = PDF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn serialize(from: &PresentSnapshot) -> IoResult<IoPayload> {
        let value = serde_json::to_value(from).map_err(|error| IoError { message: format!("PresentIntoPdf: {error}"), diagnostics: Vec::new() })?;
        let wire: PdfSnapshot = serde_json::from_value(value).map_err(|error| IoError { message: format!("PresentIntoPdf: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(<PdfSnapshot as store::ArtifactPack>::encode_pack(&wire))))
    }
}
