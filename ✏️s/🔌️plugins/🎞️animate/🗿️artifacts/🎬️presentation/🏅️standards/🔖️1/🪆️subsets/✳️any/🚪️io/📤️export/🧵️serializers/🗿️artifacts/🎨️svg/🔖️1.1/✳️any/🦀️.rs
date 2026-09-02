//! 🚪️ presentation -> svg — foreign `Serializer<PresentationSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Degenerate placeholder
//! (unchanged behaviour, pre-dates this ticket): re-wraps presentation's OWN pack bytes inside an
//! `SvgSnapshot` pack container, not real SVG XML — `IoFidelity::Lossy`.

use crate::artifacts::presentation::PresentationSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::svg::SvgSnapshot;

pub const SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId::ANY };

pub struct PresentationIntoSvg;

impl Serializer<PresentationSnapshot> for PresentationIntoSvg {
    const INTO: Dialect = SVG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &PresentationSnapshot) -> IoResult<IoPayload> {
        let bytes = <PresentationSnapshot as store::ArtifactPack>::encode_pack(from);
        let wire = <SvgSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| IoError { message: format!("PresentationIntoSvg: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(<SvgSnapshot as store::ArtifactPack>::encode_pack(&wire))))
    }
}
