//! ➕️ `insert-hollow-joint` diff — inserts the row at its position, clamped to the end of the collection.

use crate::insert_hollow_joint::InsertHollowJoint;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993HollowJointDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertHollowJoint, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.hollow_joints.iter().any(|existing| existing.id == payload.hollow_joint.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Hollow joint id {} already exists.", payload.hollow_joint.id), [payload.hollow_joint.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.hollow_joints.len());
    protocol::MutationOutcome::new(En1993Diff { hollow_joints: En1993HollowJointDelta::insertion(index, payload.hollow_joint.clone()), ..Default::default() })
}
