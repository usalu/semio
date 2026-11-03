use crate::remove_fastener_position::RemoveFastenerPosition;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::{En1995FastenerDelta, En1995FastenerPatch, En1995FastenerPositionDelta};
pub fn diff(payload: &RemoveFastenerPosition, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(parent) = base.fasteners.iter().find(|row| row.id == payload.fastener_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown fastener id.", vec![payload.fastener_id.clone()]);
    };
    let Some(at) = parent.positions.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown position id.", vec![payload.fastener_id.clone(), payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1995Diff { fasteners: En1995FastenerDelta::modification(&parent.id, En1995FastenerPatch { positions: En1995FastenerPositionDelta::removal(&parent.positions, at), ..Default::default() }), ..Default::default() })
}
