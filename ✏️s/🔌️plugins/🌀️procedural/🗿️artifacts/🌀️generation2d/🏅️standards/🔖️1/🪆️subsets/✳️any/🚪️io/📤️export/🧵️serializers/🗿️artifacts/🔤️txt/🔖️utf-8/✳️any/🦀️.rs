//! 🪄️ generation2d → txt — `s.stdio.txt` is a carrier: its body is this artifact's own canonical DSL text,
//! verbatim, so the hop is `IoFidelity::Exact`.
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;

pub fn register() {}

pub fn serialize_bytes(from: &Generation2dSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    Ok(<Generation2dSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(from).into_bytes())
}
