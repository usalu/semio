//! ➕️ `insert-joint-row` diff — inserts the row at its position, clamped to the end of the collection.

use crate::insert_joint_row::InsertJointRow;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993JointRowDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertJointRow, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.joint_rows.iter().any(|existing| existing.id == payload.joint_row.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Joint row id {} already exists.", payload.joint_row.id), [payload.joint_row.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.joint_rows.len());
    protocol::MutationOutcome::new(En1993Diff { joint_rows: En1993JointRowDelta::insertion(index, payload.joint_row.clone()), ..Default::default() })
}
