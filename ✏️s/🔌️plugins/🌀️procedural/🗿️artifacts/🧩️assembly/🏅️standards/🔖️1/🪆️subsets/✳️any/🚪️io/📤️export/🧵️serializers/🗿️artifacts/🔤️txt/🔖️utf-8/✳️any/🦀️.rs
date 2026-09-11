//! 🔤️ assembly → `s.stdio.txt@utf-8` — the document's own `.assembly` DSL text, losslessly.
//!
//! This artifact's native serialization already is UTF-8 text (`ArtifactDsl::print_dsl`).
use crate::AssemblySnapshot;
use semio_s_artifact_stdio_txt::TxtSnapshot;

pub fn register() {}

pub fn serialize(snapshot: &AssemblySnapshot) -> Result<TxtSnapshot, store::TextError> {
    Ok(TxtSnapshot::from_body(&<AssemblySnapshot as store::ArtifactDsl>::print_dsl(snapshot)))
}

pub fn serialize_bytes(snapshot: &AssemblySnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(serialize(snapshot)?.to_body().into_bytes())
}
