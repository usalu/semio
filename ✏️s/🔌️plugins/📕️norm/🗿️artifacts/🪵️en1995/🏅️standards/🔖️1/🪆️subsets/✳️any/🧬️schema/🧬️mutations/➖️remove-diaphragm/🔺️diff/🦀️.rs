use crate::remove_diaphragm::RemoveDiaphragm;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::En1995DiaphragmDelta;
pub fn diff(payload: &RemoveDiaphragm, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if payload.index >= base.diaphragms.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Diaphragm index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1995Diff { diaphragms: En1995DiaphragmDelta::removal(&base.diaphragms, payload.index), ..Default::default() })
}
