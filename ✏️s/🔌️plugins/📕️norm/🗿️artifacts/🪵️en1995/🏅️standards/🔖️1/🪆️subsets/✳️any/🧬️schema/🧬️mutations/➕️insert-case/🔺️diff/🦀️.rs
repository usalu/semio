use crate::insert_case::InsertCase;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_kernel::En1995Cases;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn diff(payload: &InsertCase, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    match semio_s_plugin_norm_compliance::design_case::add_case::<En1995Cases>(&base.cases, payload.index, &payload.case) {
        Ok(cases) => protocol::MutationOutcome::new(En1995Diff { cases, ..Default::default() }),
        Err(refusal) => protocol::MutationOutcome::fatal(refusal.code(), refusal.message(), refusal.targets()),
    }
}
