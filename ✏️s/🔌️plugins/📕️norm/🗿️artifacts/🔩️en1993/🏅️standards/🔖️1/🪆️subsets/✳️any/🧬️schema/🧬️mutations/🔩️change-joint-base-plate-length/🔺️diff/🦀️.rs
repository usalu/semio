//! 🔺️ `change-joint-base-plate-length` diff: patches `base_plate_length` of the addressed joint row.

use crate::change_joint_base_plate_length::ChangeJointBasePlateLength;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993JointDelta, En1993JointPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeJointBasePlateLength, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_base_plate_length.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.joints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "joint", Vec::<String>::new());
    };
    if row.base_plate_length == payload.new_base_plate_length {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The joint already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { joints: En1993JointDelta::modification(payload.id.clone(), En1993JointPatch { base_plate_length: Some(payload.new_base_plate_length), ..Default::default() }), ..Default::default() })
}
