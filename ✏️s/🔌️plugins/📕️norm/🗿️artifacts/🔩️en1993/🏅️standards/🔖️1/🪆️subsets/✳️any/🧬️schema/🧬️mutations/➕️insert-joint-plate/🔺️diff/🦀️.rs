//! ➕️ `insert-joint-plate` diff — inserts the row at its position, clamped to the end of the collection.

use crate::insert_joint_plate::InsertJointPlate;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993JointPlateDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertJointPlate, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.joint_plates.iter().any(|existing| existing.id == payload.joint_plate.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Joint plate id {} already exists.", payload.joint_plate.id), [payload.joint_plate.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.joint_plates.len());
    protocol::MutationOutcome::new(En1993Diff { joint_plates: En1993JointPlateDelta::insertion(index, payload.joint_plate.clone()), ..Default::default() })
}
