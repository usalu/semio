use crate::insert_fastener_position::InsertFastenerPosition;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::{En1995FastenerDelta, En1995FastenerPatch, En1995FastenerPositionDelta};
pub fn diff(payload: &InsertFastenerPosition, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(parent) = base.fasteners.iter().find(|row| row.id == payload.fastener_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown fastener id.", vec![payload.fastener_id.clone()]);
    };
    if parent.positions.iter().any(|row| row.id == payload.position.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("FastenerPosition id {} already exists.", payload.position.id), [payload.position.id.clone()]);
    }
    let at = payload.index.min(parent.positions.len());
    protocol::MutationOutcome::new(En1995Diff { fasteners: En1995FastenerDelta::modification(&parent.id, En1995FastenerPatch { positions: En1995FastenerPositionDelta::insertion(at, payload.position.clone()), ..Default::default() }), ..Default::default() })
}
