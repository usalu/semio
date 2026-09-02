//! 🚪️ present -> png — foreign `Serializer<PresentSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Degenerate placeholder
//! (unchanged behaviour, pre-dates this ticket): re-wraps present's OWN pack bytes inside a
//! `PngSnapshot` pack container, not real PNG raster data — `IoFidelity::Lossy`.

use crate::artifacts::present::PresentSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::png::PngSnapshot;

pub const PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

pub struct PresentIntoPng;

impl Serializer<PresentSnapshot> for PresentIntoPng {
    const INTO: Dialect = PNG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &PresentSnapshot) -> IoResult<IoPayload> {
        let bytes = <PresentSnapshot as store::ArtifactPack>::encode_pack(from);
        let wire = <PngSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| IoError { message: format!("PresentIntoPng: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(<PngSnapshot as store::ArtifactPack>::encode_pack(&wire))))
    }
}
