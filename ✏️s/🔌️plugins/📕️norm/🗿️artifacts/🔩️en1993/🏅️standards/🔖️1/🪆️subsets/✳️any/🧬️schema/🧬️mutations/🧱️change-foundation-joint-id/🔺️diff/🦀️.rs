//! 🔺️ `change-foundation-joint-id` diff: patches `joint_id` of the addressed foundation block row.

use crate::change_foundation_joint_id::ChangeFoundationJointId;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FoundationBlockDelta, En1993FoundationBlockPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFoundationJointId, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.foundation_blocks.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "foundation-block", Vec::<String>::new());
    };
    if row.joint_id == payload.new_joint_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The foundation block already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { foundation_blocks: En1993FoundationBlockDelta::modification(payload.id.clone(), En1993FoundationBlockPatch { joint_id: Some(payload.new_joint_id.clone()), ..Default::default() }), ..Default::default() })
}
