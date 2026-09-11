//! 🔤️ assembly ← `s.stdio.txt@utf-8` — the exact inverse of the txt export leaf.
use crate::AssemblySnapshot;
use semio_s_artifact_stdio_txt::TxtSnapshot;

pub fn register() {}

pub fn deserialize(from: &TxtSnapshot) -> Result<AssemblySnapshot, store::TextError> {
    <AssemblySnapshot as store::ArtifactDsl>::parse_dsl(&from.to_body())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<AssemblySnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|error| store::TextError::new(format!("assembly←txt: not valid utf-8: {error}"), dsl::TextSpan::at(1, 1)))?;
    deserialize(&TxtSnapshot::from_body(text))
}
