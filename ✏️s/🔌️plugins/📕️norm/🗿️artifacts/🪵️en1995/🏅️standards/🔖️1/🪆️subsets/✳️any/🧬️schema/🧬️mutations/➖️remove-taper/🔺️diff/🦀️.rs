use crate::remove_taper::RemoveTaper;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::En1995TaperDelta;
pub fn diff(payload: &RemoveTaper, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if payload.index >= base.tapers.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Taper index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1995Diff { tapers: En1995TaperDelta::removal(&base.tapers, payload.index), ..Default::default() })
}
