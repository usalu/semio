//! 🔤️ wfc3d → `s.stdio.txt@utf-8` — the document's own `.wfc3d` DSL text, losslessly.
//!
//! This artifact's native serialization already is UTF-8 text (`ArtifactDsl::print_dsl`).
use crate::Wfc3dSnapshot;
use semio_s_artifact_stdio_txt::TxtSnapshot;

pub fn register() {}

pub fn serialize(snapshot: &Wfc3dSnapshot) -> Result<TxtSnapshot, store::TextError> {
    Ok(TxtSnapshot::from_body(&<Wfc3dSnapshot as store::ArtifactDsl>::print_dsl(snapshot)))
}

pub fn serialize_bytes(snapshot: &Wfc3dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(serialize(snapshot)?.to_body().into_bytes())
}
