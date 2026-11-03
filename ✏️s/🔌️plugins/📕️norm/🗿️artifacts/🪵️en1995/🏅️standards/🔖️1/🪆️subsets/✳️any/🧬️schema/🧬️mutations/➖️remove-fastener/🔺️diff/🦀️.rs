use crate::remove_fastener::RemoveFastener;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::En1995FastenerDelta;
pub fn diff(payload: &RemoveFastener, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if payload.index >= base.fasteners.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Fastener index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1995Diff { fasteners: En1995FastenerDelta::removal(&base.fasteners, payload.index), ..Default::default() })
}
