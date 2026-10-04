//! 🗺️ gismap ← txt — the body of a `s.stdio.txt` carrier parsed as this artifact's own DSL, the
//! inverse of the sibling export leaf (`IoFidelity::Exact`).
use crate::GisMapSnapshot;
use semio_s_artifact_stdio_txt::TxtSnapshot;

pub fn register() {}

pub fn deserialize(from: &TxtSnapshot) -> Result<GisMapSnapshot, semio_framework_diagnostic::TextError> {
    <GisMapSnapshot as store::ArtifactDsl>::parse_dsl(&from.to_body())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<GisMapSnapshot, semio_framework_diagnostic::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("gismap←txt: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    <GisMapSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
