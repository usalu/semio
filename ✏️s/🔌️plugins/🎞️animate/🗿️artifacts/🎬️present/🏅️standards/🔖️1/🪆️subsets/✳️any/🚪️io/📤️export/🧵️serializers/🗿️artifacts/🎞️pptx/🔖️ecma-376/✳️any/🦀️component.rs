//! 🚪️ present -> pptx — foreign `Serializer<PresentSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Structural `serde_json`
//! coercion between `PresentSnapshot`'s and `PptxSnapshot`'s (unrelated) field shapes — not a real
//! present->pptx semantic mapping (unchanged behaviour, pre-dates this ticket) — `IoFidelity::Lossy`.

use crate::artifacts::present::PresentSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::pptx::PptxSnapshot;

pub const PPTX_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId::ANY };

pub struct PresentIntoPptx;

impl Serializer<PresentSnapshot> for PresentIntoPptx {
    const INTO: Dialect = PPTX_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn serialize(from: &PresentSnapshot) -> IoResult<IoPayload> {
        let value = serde_json::to_value(from).map_err(|error| IoError { message: format!("PresentIntoPptx: {error}"), diagnostics: Vec::new() })?;
        let wire: PptxSnapshot = serde_json::from_value(value).map_err(|error| IoError { message: format!("PresentIntoPptx: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(<PptxSnapshot as store::ArtifactPack>::encode_pack(&wire))))
    }
}
