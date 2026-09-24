//! generation2d → txt — `s.stdio.txt` is a carrier: its body is this artifact's own canonical DSL text,
//! verbatim, so the hop is `IoFidelity::Exact`.
use crate::Generation2dSnapshot;

pub fn register() {}

pub fn serialize_bytes(from: &Generation2dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<Generation2dSnapshot as store::ArtifactDsl>::print_dsl(from).into_bytes())
}
