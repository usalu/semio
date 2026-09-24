//! shooting ← txt — the body of a `s.stdio.txt` carrier parsed as this artifact's own DSL, the inverse
//! of the sibling export leaf (`IoFidelity::Exact`).
use crate::ShootingSnapshot;

pub fn register() {}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<ShootingSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|error| store::TextError::new(format!("shooting←txt: {error}"), dsl::TextSpan::at(1, 1)))?;
    <ShootingSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
