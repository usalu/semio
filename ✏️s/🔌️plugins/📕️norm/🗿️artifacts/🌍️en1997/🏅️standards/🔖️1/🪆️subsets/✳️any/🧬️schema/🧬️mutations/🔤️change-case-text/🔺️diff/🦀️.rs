use crate::change_case_text::ChangeCaseText;
use semio_s_artifact_norm_en1997_diff::En1997Diff;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn diff(payload: &ChangeCaseText, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    match semio_s_plugin_norm_compliance::design_case::change_case_text(&base.cases, &payload.case_id, &payload.key, payload.value.as_deref()) {
        Ok(cases) => protocol::MutationOutcome::new(En1997Diff { cases, ..Default::default() }),
        Err(refusal) => protocol::MutationOutcome::fatal(refusal.code(), refusal.message(), refusal.targets()),
    }
}
