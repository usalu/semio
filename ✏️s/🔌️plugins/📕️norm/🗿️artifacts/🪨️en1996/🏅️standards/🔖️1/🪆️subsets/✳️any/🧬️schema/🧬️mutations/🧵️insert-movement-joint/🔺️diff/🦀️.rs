//! 🧵 `insert-movement-joint` diff: inserts the movement joint row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_movement_joint::InsertMovementJoint;
use semio_s_artifact_norm_en1996_diff::{En1996Diff, En1996JointDelta};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &InsertMovementJoint, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if base.movement_joints.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Movement joint {key} already exists."), [key]);
    }
    if payload.index > base.movement_joints.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.movement_joints.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1996Diff { movement_joints: En1996JointDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
