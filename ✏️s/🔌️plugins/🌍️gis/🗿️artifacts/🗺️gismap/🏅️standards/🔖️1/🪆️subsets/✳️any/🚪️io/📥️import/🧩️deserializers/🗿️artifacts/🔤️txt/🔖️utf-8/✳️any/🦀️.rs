//! 🗺️ gismap ← txt — the body of a `s.stdio.txt` carrier parsed as this artifact's own DSL, the
//! inverse of the sibling export leaf (`IoFidelity::Exact`).
use crate::GisMapSnapshot;
use semio_s_artifact_stdio_txt::TxtSnapshot;

pub fn register() {}

pub fn deserialize(from: &TxtSnapshot) -> Result<GisMapSnapshot, store::TextError> {
    <GisMapSnapshot as store::ArtifactDsl>::parse_dsl(&from.to_body())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<GisMapSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|error| store::TextError::new(format!("gismap←txt: {error}"), dsl::TextSpan::at(1, 1)))?;
    <GisMapSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
