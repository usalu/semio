use crate::change_case_text::ChangeCaseText;
use semio_s_artifact_norm_en1992_diff::En1992Diff;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;
pub fn diff(payload: &ChangeCaseText, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    match semio_s_plugin_norm_compliance::design_case::change_case_text(&base.cases, &payload.case_id, &payload.key, payload.value.as_deref()) {
        Ok(cases) => protocol::MutationOutcome::new(En1992Diff { cases, ..Default::default() }),
        Err(refusal) => protocol::MutationOutcome::fatal(refusal.code(), refusal.message(), refusal.targets()),
    }
}
