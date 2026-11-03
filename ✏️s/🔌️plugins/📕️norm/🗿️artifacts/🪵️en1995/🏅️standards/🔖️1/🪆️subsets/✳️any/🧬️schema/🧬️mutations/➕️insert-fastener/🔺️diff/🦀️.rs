use crate::insert_fastener::InsertFastener;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::En1995FastenerDelta;
pub fn diff(payload: &InsertFastener, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if base.fasteners.iter().any(|existing| existing.id == payload.fastener.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Fastener id {} already exists.", payload.fastener.id), [payload.fastener.id.clone()]);
    }
    let at = payload.index.min(base.fasteners.len());
    protocol::MutationOutcome::new(En1995Diff { fasteners: En1995FastenerDelta::insertion(at, payload.fastener.clone()), ..Default::default() })
}
