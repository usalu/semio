//! 🧵 `change-movement-joint-position` diff: patches `position_m` of the addressed movement joint row.

use crate::change_movement_joint_position::ChangeMovementJointPosition;
use semio_s_artifact_norm_en1996_diff::{En1996Diff, En1996JointDelta, En1996JointPatch};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &ChangeMovementJointPosition, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_position_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.movement_joints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Movement joint {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.position_m == payload.new_position_m {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The movement joint already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { movement_joints: En1996JointDelta::modification(&payload.id, En1996JointPatch { position_m: Some(payload.new_position_m), ..Default::default() }), ..Default::default() })
}
