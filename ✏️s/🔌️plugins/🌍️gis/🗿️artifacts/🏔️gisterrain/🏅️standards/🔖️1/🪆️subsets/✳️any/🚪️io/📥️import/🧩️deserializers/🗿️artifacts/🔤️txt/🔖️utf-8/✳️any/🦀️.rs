//! 🏔️ gisterrain ← txt — the body of a `s.stdio.txt` carrier parsed as this artifact's own DSL, the
//! inverse of the sibling export leaf (`IoFidelity::Exact`).
use crate::GisTerrainSnapshot;
use semio_s_artifact_stdio_txt::TxtSnapshot;

pub fn register() {}

pub fn deserialize(from: &TxtSnapshot) -> Result<GisTerrainSnapshot, store::TextError> {
    <GisTerrainSnapshot as store::ArtifactDsl>::parse_dsl(&from.to_body())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<GisTerrainSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|error| store::TextError::new(format!("gisterrain←txt: {error}"), dsl::TextSpan::at(1, 1)))?;
    <GisTerrainSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
