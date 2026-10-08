//! 🔩️ `update-bolt-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdateBoltInputs;
use crate::diff::{En1993Diff, En1993JointDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdateBoltInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let delta = match base.joints.iter().position(|row| row.id == payload.joint.id) {
        Some(index) if base.joints[index] == payload.joint => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => {
            let mut replacement = En1993JointDelta::removal(&payload.joint.id);
            replacement.absorb(En1993JointDelta::insertion(&base.joints, index, payload.joint.clone()));
            replacement
        }
        None => En1993JointDelta::insertion(&base.joints, base.joints.len(), payload.joint.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { joints: delta, ..Default::default() })
}
