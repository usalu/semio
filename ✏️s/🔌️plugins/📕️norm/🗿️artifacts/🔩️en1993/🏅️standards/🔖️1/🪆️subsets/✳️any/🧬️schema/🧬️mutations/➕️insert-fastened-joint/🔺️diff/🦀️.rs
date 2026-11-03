//! ➕️ `insert-fastened-joint` diff — inserts the row at its position, clamped to the end of the collection.

use crate::insert_fastened_joint::InsertFastenedJoint;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FastenedJointDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertFastenedJoint, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.fastened_joints.iter().any(|existing| existing.id == payload.fastened_joint.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Fastened joint id {} already exists.", payload.fastened_joint.id), [payload.fastened_joint.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.fastened_joints.len());
    protocol::MutationOutcome::new(En1993Diff { fastened_joints: En1993FastenedJointDelta::insertion(index, payload.fastened_joint.clone()), ..Default::default() })
}
