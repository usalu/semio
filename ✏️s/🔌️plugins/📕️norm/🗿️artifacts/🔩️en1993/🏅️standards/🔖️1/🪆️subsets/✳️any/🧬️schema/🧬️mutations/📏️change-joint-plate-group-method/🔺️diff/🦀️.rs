//! 🔺️ `change-joint-plate-group-method` diff: patches `group_method` of the addressed joint_plate row.

use crate::change_joint_plate_group_method::ChangeJointPlateGroupMethod;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993JointPlateDelta, En1993JointPlatePatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeJointPlateGroupMethod, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.joint_plates.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "joint_plate", Vec::<String>::new());
    };
    if row.group_method == payload.new_group_method {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The joint_plate already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { joint_plates: En1993JointPlateDelta::modification(payload.id.clone(), En1993JointPlatePatch { group_method: Some(payload.new_group_method), ..Default::default() }), ..Default::default() })
}
