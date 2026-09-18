//! 🔤️ wfc3d ← `s.stdio.txt@utf-8` — the exact inverse of the txt export leaf.
use crate::Wfc3dSnapshot;
use semio_s_artifact_stdio_txt::TxtSnapshot;

pub fn register() {}

pub fn deserialize(from: &TxtSnapshot) -> Result<Wfc3dSnapshot, store::TextError> {
    <Wfc3dSnapshot as store::ArtifactDsl>::parse_dsl(&from.to_body())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Wfc3dSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|error| store::TextError::new(format!("wfc3d←txt: not valid utf-8: {error}"), dsl::TextSpan::at(1, 1)))?;
    deserialize(&TxtSnapshot::from_body(text))
}
