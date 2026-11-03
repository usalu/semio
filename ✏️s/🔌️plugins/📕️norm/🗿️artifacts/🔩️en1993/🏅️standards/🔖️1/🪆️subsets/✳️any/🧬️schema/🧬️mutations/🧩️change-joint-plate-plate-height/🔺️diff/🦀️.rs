//! 🔺️ `change-joint-plate-plate-height` diff: patches `plate_height` of the addressed joint plate row.

use crate::change_joint_plate_plate_height::ChangeJointPlatePlateHeight;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993JointPlateDelta, En1993JointPlatePatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeJointPlatePlateHeight, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_plate_height.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.joint_plates.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "joint-plate", Vec::<String>::new());
    };
    if row.plate_height == payload.new_plate_height {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The joint plate already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { joint_plates: En1993JointPlateDelta::modification(payload.id.clone(), En1993JointPlatePatch { plate_height: Some(payload.new_plate_height), ..Default::default() }), ..Default::default() })
}
