//! 🚪️ present <- png — foreign `Deserializer<PresentSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Degenerate placeholder
//! (unchanged behaviour, pre-dates this ticket): decodes the payload directly as present's OWN
//! pack/dsl bytes, not real PNG raster data — `IoFidelity::Lossy`.

use crate::artifacts::present::PresentSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

pub struct PngIntoPresent;

impl Deserializer<PresentSnapshot> for PngIntoPresent {
    const FROM: Dialect = PNG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(payload: &IoPayload) -> IoResult<PresentSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "PngIntoPresent: expected a binary png payload".to_string(), diagnostics: Vec::new() });
        };
        let snapshot = <PresentSnapshot as store::ArtifactPack>::decode_pack(bytes)
            .or_else(|_| <PresentSnapshot as store::ArtifactDsl>::parse_dsl(&String::from_utf8_lossy(bytes)))
            .map_err(|error| IoError { message: format!("PngIntoPresent: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(snapshot).await)
    }
}
