use crate::insert_case::InsertCase;
use semio_s_artifact_norm_en1996_diff::En1996Diff;
use semio_s_artifact_norm_en1996_masonry::En1996Cases;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn diff(payload: &InsertCase, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    match semio_s_plugin_norm_compliance::design_case::add_case::<En1996Cases>(&base.cases, payload.index, &payload.case) {
        Ok(cases) => protocol::MutationOutcome::new(En1996Diff { cases, ..Default::default() }),
        Err(refusal) => protocol::MutationOutcome::fatal(refusal.code(), refusal.message(), refusal.targets()),
    }
}
