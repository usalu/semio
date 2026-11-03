//! 🔺️ `change-joint-beam-inertia` diff: patches `beam_inertia` of the addressed joint row.

use crate::change_joint_beam_inertia::ChangeJointBeamInertia;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993JointDelta, En1993JointPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeJointBeamInertia, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_beam_inertia.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.joints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "joint", Vec::<String>::new());
    };
    if row.beam_inertia == payload.new_beam_inertia {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The joint already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { joints: En1993JointDelta::modification(payload.id.clone(), En1993JointPatch { beam_inertia: Some(payload.new_beam_inertia), ..Default::default() }), ..Default::default() })
}
