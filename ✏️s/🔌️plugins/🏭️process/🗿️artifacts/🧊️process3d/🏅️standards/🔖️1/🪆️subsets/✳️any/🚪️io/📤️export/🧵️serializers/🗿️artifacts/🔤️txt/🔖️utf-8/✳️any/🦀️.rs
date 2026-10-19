//! ⚙️ process3d → txt — `s.stdio.txt` is a carrier: its body is this artifact's own canonical DSL text,
//! verbatim, so the hop is `IoFidelity::Exact`.
use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;

pub fn register() {}

pub fn serialize_bytes(from: &Process3dSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    Ok(<Process3dSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(from).into_bytes())
}
