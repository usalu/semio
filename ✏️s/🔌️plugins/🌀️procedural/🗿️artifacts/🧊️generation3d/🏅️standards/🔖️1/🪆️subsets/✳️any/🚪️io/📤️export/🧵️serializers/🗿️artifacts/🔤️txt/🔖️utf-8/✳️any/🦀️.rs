//! 🔤️ generation3d → `s.stdio.txt@utf-8` — the document's own `🗣️.dsl.semio` text, losslessly.
//!
//! 🐛️ Before ticket 26/09/09/PROCEDURAL-3D-END-TO-END both halves of this leaf were
//! `Err("txt export not yet implemented")`, and this file also carried a stray `deserialize_bytes`
//! that belonged to the import side.
//!
//! There is nothing to invent: this artifact's NATIVE serialization already IS UTF-8 text —
//! `ArtifactDsl::print_dsl`, the same grammar `derived_analysis`/`Generation3dBuilder::from_text`
//! parse back. So `txt` is the one FULL-FIDELITY target in this artifact's IO surface (every mesh
//! format keeps only the evaluated geometry; txt keeps the whole flow graph and generation
//! history), and the paired import leaf is its exact inverse.
use crate::Generation3dSnapshot;
use semio_s_artifact_stdio_txt::TxtSnapshot;

pub fn register() {}

pub fn serialize(snapshot: &Generation3dSnapshot) -> Result<TxtSnapshot, store::TextError> {
    Ok(TxtSnapshot::from_body(&<Generation3dSnapshot as store::ArtifactDsl>::print_dsl(snapshot)))
}

pub fn serialize_bytes(snapshot: &Generation3dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(serialize(snapshot)?.to_body().into_bytes())
}
