//! 🔺️ `change-hollow-brace-fy` diff: patches `brace_fy` of the addressed hollow joint row.

use crate::change_hollow_brace_fy::ChangeHollowBraceFy;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993HollowJointDelta, En1993HollowJointPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeHollowBraceFy, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_brace_fy.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.hollow_joints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "hollow-joint", Vec::<String>::new());
    };
    if row.brace_fy == payload.new_brace_fy {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The hollow joint already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { hollow_joints: En1993HollowJointDelta::modification(payload.id.clone(), En1993HollowJointPatch { brace_fy: Some(payload.new_brace_fy), ..Default::default() }), ..Default::default() })
}
