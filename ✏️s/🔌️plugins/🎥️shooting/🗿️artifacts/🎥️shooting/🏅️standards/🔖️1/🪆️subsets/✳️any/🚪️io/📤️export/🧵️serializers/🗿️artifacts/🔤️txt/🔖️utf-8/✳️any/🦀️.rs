//! shooting → txt — `s.stdio.txt` is a carrier: its body is this artifact's own canonical DSL text,
//! verbatim, so the hop is `IoFidelity::Exact`.
use crate::ShootingSnapshot;

pub fn register() {}

pub fn serialize_bytes(from: &ShootingSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<ShootingSnapshot as store::ArtifactDsl>::print_dsl(from).into_bytes())
}
