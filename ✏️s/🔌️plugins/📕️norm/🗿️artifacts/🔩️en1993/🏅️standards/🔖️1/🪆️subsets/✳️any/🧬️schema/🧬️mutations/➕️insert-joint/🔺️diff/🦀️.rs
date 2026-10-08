//! ➕️ `insert-joint` diff — inserts the row at its position, clamped to the end of the collection.

use super::InsertJoint;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993JointEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &InsertJoint, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.joints.iter().any(|existing| existing.id == payload.joint.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Joint id {} already exists.", payload.joint.id), [payload.joint.id.clone()]);
    }
    let index = payload.index.min(base.joints.len());
    protocol::MutationOutcome::new(En1993Diff { joints: vec![En1993JointEdit::insert(index, payload.joint.clone())], ..Default::default() })
}
