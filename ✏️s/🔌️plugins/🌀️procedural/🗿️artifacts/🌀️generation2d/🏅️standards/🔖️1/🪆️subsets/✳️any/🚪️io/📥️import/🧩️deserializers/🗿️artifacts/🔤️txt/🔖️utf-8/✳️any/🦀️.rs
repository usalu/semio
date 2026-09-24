//! generation2d ← txt — the body of a `s.stdio.txt` carrier parsed as this artifact's own DSL, the inverse
//! of the sibling export leaf (`IoFidelity::Exact`).
use crate::Generation2dSnapshot;

pub fn register() {}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Generation2dSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|error| store::TextError::new(format!("generation2d←txt: {error}"), dsl::TextSpan::at(1, 1)))?;
    <Generation2dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
