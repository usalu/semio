use crate::insert_case::InsertCase;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_checks::catalogue::En1991Cases;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &InsertCase, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    match semio_s_plugin_norm_compliance::design_case::add_case::<En1991Cases>(&base.cases, payload.index, &payload.case) {
        Ok(cases) => protocol::MutationOutcome::new(En1991Diff { cases, ..Default::default() }),
        Err(refusal) => protocol::MutationOutcome::fatal(refusal.code(), refusal.message(), refusal.targets()),
    }
}
