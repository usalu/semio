//! 🚪️ presentation <- pptx — foreign `Deserializer<PresentationSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Structural `serde_json`
//! coercion between `PptxSnapshot`'s and `PresentationSnapshot`'s (unrelated) field shapes — not a real
//! pptx->presentation semantic mapping (unchanged behaviour, pre-dates this ticket) — `IoFidelity::Lossy`.

use crate::artifacts::presentation::PresentationSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::pptx::PptxSnapshot;

pub const PPTX_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId::ANY };

pub struct PptxIntoPresentation;

impl Deserializer<PresentationSnapshot> for PptxIntoPresentation {
    const FROM: Dialect = PPTX_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(payload: &IoPayload) -> IoResult<PresentationSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "PptxIntoPresentation: expected a binary pptx payload".to_string(), diagnostics: Vec::new() });
        };
        let wire = <PptxSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError { message: format!("PptxIntoPresentation: {error}"), diagnostics: Vec::new() })?;
        let value = serde_json::to_value(&wire).map_err(|error| IoError { message: format!("PptxIntoPresentation: {error}"), diagnostics: Vec::new() })?;
        let dsl_value: dsl::DslValue = value.into();
        let snapshot: PresentationSnapshot = dsl::FromValue::from_value(dsl_value).map_err(|error| IoError { message: format!("PptxIntoPresentation: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(snapshot))
    }
}
