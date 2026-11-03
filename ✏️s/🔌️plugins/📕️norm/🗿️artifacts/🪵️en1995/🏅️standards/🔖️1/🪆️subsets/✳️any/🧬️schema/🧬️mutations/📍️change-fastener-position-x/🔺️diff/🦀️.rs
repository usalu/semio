use crate::change_fastener_position_x::ChangeFastenerPositionX;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::{En1995FastenerDelta, En1995FastenerPatch, En1995FastenerPositionDelta, En1995FastenerPositionPatch};
pub fn diff(payload: &ChangeFastenerPositionX, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(parent) = base.fasteners.iter().find(|row| row.id == payload.fastener_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown fastener id.", vec![payload.fastener_id.clone()]);
    };
    let Some(row) = parent.positions.iter().find(|row| row.id == payload.position_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown position id.", vec![payload.fastener_id.clone(), payload.position_id.clone()]);
    };
    if !payload.new_value.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1995Diff { fasteners: En1995FastenerDelta::modification(&parent.id, En1995FastenerPatch { positions: En1995FastenerPositionDelta::modification(&row.id, En1995FastenerPositionPatch { x_m: Some(payload.new_value), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
