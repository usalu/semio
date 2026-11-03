use crate::insert_case::InsertCase;
use semio_s_artifact_norm_en1994_diff::En1994Diff;
use semio_s_artifact_norm_en1994_checks::case_catalogue::En1994Cases;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn diff(payload: &InsertCase, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    match semio_s_plugin_norm_compliance::design_case::add_case::<En1994Cases>(&base.cases, payload.index, &payload.case) {
        Ok(cases) => protocol::MutationOutcome::new(En1994Diff { cases, ..Default::default() }),
        Err(refusal) => protocol::MutationOutcome::fatal(refusal.code(), refusal.message(), refusal.targets()),
    }
}
