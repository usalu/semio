//! 🚪️ presentation -> pptx — foreign `Serializer<PresentationSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Structural `dsl::DslValue`
//! coercion between `PresentationSnapshot`'s and `PptxSnapshot`'s (unrelated) field shapes — not a real
//! presentation->pptx semantic mapping (unchanged behaviour, pre-dates this ticket) — `IoFidelity::Lossy`.

use crate::artifacts::presentation::PresentationSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::pptx::PptxSnapshot;

pub const PPTX_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId::ANY };

pub struct PresentationIntoPptx;

impl Serializer<PresentationSnapshot> for PresentationIntoPptx {
    const INTO: Dialect = PPTX_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &PresentationSnapshot) -> IoResult<IoPayload> {
        let value = dsl::ToValue::to_value(from);
        let wire: PptxSnapshot = dsl::FromValue::from_value(value).map_err(|error| IoError { message: format!("PresentationIntoPptx: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(<PptxSnapshot as store::ArtifactPack>::encode_pack(&wire))))
    }
}
