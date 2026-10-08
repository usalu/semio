//! 🔩️ `update-bolt-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdateBoltInputs;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993JointEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdateBoltInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let edit = match base.joints.iter().position(|row| row.id == payload.joint.id) {
        Some(index) if base.joints[index] == payload.joint => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => En1993JointEdit::replace(index, payload.joint.id.clone(), payload.joint.clone()),
        None => En1993JointEdit::insert(base.joints.len(), payload.joint.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { joints: vec![edit], ..Default::default() })
}
