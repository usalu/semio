//! ✏️ `change-joint-beam-moment-rd-nm` diff: patches `beam_moment_rd_nm` of the addressed joint row.

use crate::change_joint_beam_moment_rd_nm::ChangeJointBeamMomentRdNm;
use semio_s_artifact_norm_en1998_diff::{En1998JointDelta, En1998JointPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeJointBeamMomentRdNm, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_beam_moment_rd_nm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.joints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("joint {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.beam_moment_rd_nm == payload.new_beam_moment_rd_nm {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The joint already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { joints: En1998JointDelta::modification(&payload.id, En1998JointPatch { beam_moment_rd_nm: Some(payload.new_beam_moment_rd_nm), ..Default::default() }), ..Default::default() })
}
