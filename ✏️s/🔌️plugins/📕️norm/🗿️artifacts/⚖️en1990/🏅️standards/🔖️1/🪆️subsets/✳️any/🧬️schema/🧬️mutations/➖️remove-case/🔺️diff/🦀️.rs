use crate::remove_case::RemoveCase;
use semio_s_artifact_norm_en1990_diff::En1990Diff;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
pub fn diff(payload: &RemoveCase, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    match semio_s_plugin_norm_compliance::design_case::remove_case(&base.cases, &payload.id) {
        Ok(cases) => protocol::MutationOutcome::new(En1990Diff { cases, ..Default::default() }),
        Err(refusal) => protocol::MutationOutcome::fatal(refusal.code(), refusal.message(), refusal.targets()),
    }
}
