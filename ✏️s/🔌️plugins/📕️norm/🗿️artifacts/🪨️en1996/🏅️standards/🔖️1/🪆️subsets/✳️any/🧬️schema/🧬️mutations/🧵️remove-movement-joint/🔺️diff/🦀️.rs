//! 🧵 `remove-movement-joint` diff: removes the addressed movement joint row.

use crate::remove_movement_joint::RemoveMovementJoint;
use semio_s_artifact_norm_en1996_diff::{En1996Diff, En1996JointDelta};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &RemoveMovementJoint, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    let Some(index) = base.movement_joints.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Movement joint {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1996Diff { movement_joints: En1996JointDelta::removal(&base.movement_joints, index), ..Default::default() })
}
