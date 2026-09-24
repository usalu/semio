//! 🏔️ gisterrain → txt — `s.stdio.txt` is a carrier: its body is this artifact's own canonical DSL
//! text, verbatim, so the hop is `IoFidelity::Exact`.
use crate::GisTerrainSnapshot;
use semio_s_artifact_stdio_txt::TxtSnapshot;

pub fn register() {}

pub fn serialize(from: &GisTerrainSnapshot) -> Result<TxtSnapshot, store::TextError> {
    Ok(TxtSnapshot::from_body(&<GisTerrainSnapshot as store::ArtifactDsl>::print_dsl(from)))
}

pub fn serialize_bytes(from: &GisTerrainSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(serialize(from)?.to_body().into_bytes())
}
