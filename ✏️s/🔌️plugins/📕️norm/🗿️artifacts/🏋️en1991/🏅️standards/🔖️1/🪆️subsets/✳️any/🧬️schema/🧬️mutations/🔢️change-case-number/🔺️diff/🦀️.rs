use crate::change_case_number::ChangeCaseNumber;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeCaseNumber, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    match semio_s_plugin_norm_compliance::design_case::change_case_number(&base.cases, &payload.case_id, &payload.key, payload.value) {
        Ok(cases) => protocol::MutationOutcome::new(En1991Diff { cases, ..Default::default() }),
        Err(refusal) => protocol::MutationOutcome::fatal(refusal.code(), refusal.message(), refusal.targets()),
    }
}
