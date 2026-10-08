//! ➖️ `remove-joint` diff — removes the row at the index, guarded by the row's own id.

use super::RemoveJoint;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993JointEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &RemoveJoint, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.joints.get(payload.index) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("joint index {} out of range.", payload.index), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1993Diff { joints: vec![En1993JointEdit::remove(payload.index, row.id.clone())], ..Default::default() })
}
