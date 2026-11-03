//! ➖️ `remove-joint-plate` diff — removes the row at the index.

use crate::remove_joint_plate::RemoveJointPlate;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993JointPlateDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveJointPlate, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.joint_plates.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("joint-plate index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { joint_plates: En1993JointPlateDelta::removal(&base.joint_plates, payload.index), ..Default::default() })
}
