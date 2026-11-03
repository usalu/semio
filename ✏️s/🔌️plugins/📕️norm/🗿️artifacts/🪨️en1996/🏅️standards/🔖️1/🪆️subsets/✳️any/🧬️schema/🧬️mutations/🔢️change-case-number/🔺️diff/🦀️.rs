use crate::change_case_number::ChangeCaseNumber;
use semio_s_artifact_norm_en1996_diff::En1996Diff;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn diff(payload: &ChangeCaseNumber, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    match semio_s_plugin_norm_compliance::design_case::change_case_number(&base.cases, &payload.case_id, &payload.key, payload.value) {
        Ok(cases) => protocol::MutationOutcome::new(En1996Diff { cases, ..Default::default() }),
        Err(refusal) => protocol::MutationOutcome::fatal(refusal.code(), refusal.message(), refusal.targets()),
    }
}
