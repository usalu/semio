//! flow ← txt — the body of a `s.stdio.txt` carrier parsed as this artifact's own DSL, the inverse
//! of the sibling export leaf (`IoFidelity::Exact`).
use crate::FlowSnapshot;

pub fn register() {}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<FlowSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|error| store::TextError::new(format!("flow←txt: {error}"), dsl::TextSpan::at(1, 1)))?;
    <FlowSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
