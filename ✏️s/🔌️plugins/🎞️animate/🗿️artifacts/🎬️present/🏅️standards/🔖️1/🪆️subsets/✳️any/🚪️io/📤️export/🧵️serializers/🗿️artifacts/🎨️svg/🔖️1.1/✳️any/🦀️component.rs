//! 🚪️ present -> svg — foreign `Serializer<PresentSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Degenerate placeholder
//! (unchanged behaviour, pre-dates this ticket): re-wraps present's OWN pack bytes inside an
//! `SvgSnapshot` pack container, not real SVG XML — `IoFidelity::Lossy`.

use crate::artifacts::present::PresentSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::svg::SvgSnapshot;

pub const SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId::ANY };

pub struct PresentIntoSvg;

impl Serializer<PresentSnapshot> for PresentIntoSvg {
    const INTO: Dialect = SVG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn serialize(from: &PresentSnapshot) -> IoResult<IoPayload> {
        let bytes = <PresentSnapshot as store::ArtifactPack>::encode_pack(from);
        let wire = <SvgSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| IoError { message: format!("PresentIntoSvg: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(<SvgSnapshot as store::ArtifactPack>::encode_pack(&wire))))
    }
}
