use crate::insert_case::InsertCase;
use semio_s_artifact_norm_en1992_diff::En1992Diff;
use semio_s_artifact_norm_en1992_checks::En1992Cases;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;
pub fn diff(payload: &InsertCase, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    match semio_s_plugin_norm_compliance::design_case::add_case::<En1992Cases>(&base.cases, payload.index, &payload.case) {
        Ok(cases) => protocol::MutationOutcome::new(En1992Diff { cases, ..Default::default() }),
        Err(refusal) => protocol::MutationOutcome::fatal(refusal.code(), refusal.message(), refusal.targets()),
    }
}
