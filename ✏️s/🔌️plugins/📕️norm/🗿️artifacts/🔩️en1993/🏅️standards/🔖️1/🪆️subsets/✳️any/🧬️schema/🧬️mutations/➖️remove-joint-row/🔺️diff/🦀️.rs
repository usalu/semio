//! ➖️ `remove-joint-row` diff — removes the row at the index.

use crate::remove_joint_row::RemoveJointRow;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993JointRowDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveJointRow, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.joint_rows.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("joint-row index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { joint_rows: En1993JointRowDelta::removal(&base.joint_rows, payload.index), ..Default::default() })
}
