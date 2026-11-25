use crate::remove_case::RemoveCase;
use semio_s_artifact_norm_en1994_diff::En1994Diff;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn diff(payload: &RemoveCase, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    match semio_s_plugin_norm_compliance::design_case::remove_case(&base.cases, &payload.id) {
        Ok(cases) => protocol::MutationOutcome::new(En1994Diff { cases, ..Default::default() }),
        Err(refusal) => protocol::MutationOutcome::fatal(refusal.code(), refusal.message(), refusal.targets()),
    }
}
