use crate::remove_case::RemoveCase;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &RemoveCase, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    match semio_s_plugin_norm_compliance::design_case::remove_case(&base.cases, &payload.id) {
        Ok(cases) => protocol::MutationOutcome::new(En1991Diff { cases, ..Default::default() }),
        Err(refusal) => protocol::MutationOutcome::fatal(refusal.code(), refusal.message(), refusal.targets()),
    }
}
