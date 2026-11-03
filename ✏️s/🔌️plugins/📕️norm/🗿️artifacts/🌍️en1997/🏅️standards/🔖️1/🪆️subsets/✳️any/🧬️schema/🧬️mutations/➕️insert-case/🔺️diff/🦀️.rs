use crate::insert_case::InsertCase;
use semio_s_artifact_norm_en1997_diff::En1997Diff;
use semio_s_artifact_norm_en1997_checks_catalogue::catalogue::En1997Cases;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn diff(payload: &InsertCase, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    match semio_s_plugin_norm_compliance::design_case::add_case::<En1997Cases>(&base.cases, payload.index, &payload.case) {
        Ok(cases) => protocol::MutationOutcome::new(En1997Diff { cases, ..Default::default() }),
        Err(refusal) => protocol::MutationOutcome::fatal(refusal.code(), refusal.message(), refusal.targets()),
    }
}
