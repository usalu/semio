//! 🔤️ generation3d ← `s.stdio.txt@utf-8` — the exact inverse of the txt export leaf.
//!
//! 🐛️ Before ticket 26/09/09/PROCEDURAL-3D-END-TO-END both halves were
//! `Err("txt import not yet implemented")`.
//!
//! The artifact's native serialization already IS UTF-8 text, so importing txt means parsing that
//! grammar: `ArtifactDsl::parse_dsl`, the same entry point `derived_analysis` and
//! `Generation3dBuilder::from_text` use — one parser, never a second reader for the same grammar.
//! This is the only FULL-FIDELITY import in this artifact's IO surface: the whole flow graph and
//! generation history come back, not just evaluated geometry.
use crate::Generation3dSnapshot;
use semio_s_artifact_stdio_txt::TxtSnapshot;

pub fn register() {}

pub fn deserialize(from: &TxtSnapshot) -> Result<Generation3dSnapshot, store::TextError> {
    <Generation3dSnapshot as store::ArtifactDsl>::parse_dsl(&from.to_body())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Generation3dSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|error| store::TextError::new(format!("generation3d←txt: not valid utf-8: {error}"), dsl::TextSpan::at(1, 1)))?;
    deserialize(&TxtSnapshot::from_body(text))
}
