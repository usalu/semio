//! 🔺️ `change-hollow-member-id` diff: patches `member_id` of the addressed hollow joint row.

use crate::change_hollow_member_id::ChangeHollowMemberId;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993HollowJointDelta, En1993HollowJointPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeHollowMemberId, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.hollow_joints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "hollow-joint", Vec::<String>::new());
    };
    if row.member_id == payload.new_member_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The hollow joint already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { hollow_joints: En1993HollowJointDelta::modification(payload.id.clone(), En1993HollowJointPatch { member_id: Some(payload.new_member_id.clone()), ..Default::default() }), ..Default::default() })
}
