use crate::change_case_number::ChangeCaseNumber;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn diff(payload: &ChangeCaseNumber, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    match semio_s_plugin_norm_compliance::design_case::change_case_number(&base.cases, &payload.case_id, &payload.key, payload.value) {
        Ok(cases) => protocol::MutationOutcome::new(En1995Diff { cases, ..Default::default() }),
        Err(refusal) => protocol::MutationOutcome::fatal(refusal.code(), refusal.message(), refusal.targets()),
    }
}
