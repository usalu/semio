//! 🔺️ `change-fatigue-member-id` diff: patches `member_id` of the addressed fatigue detail row.

use crate::change_fatigue_member_id::ChangeFatigueMemberId;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FatigueDetailDelta, En1993FatigueDetailPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFatigueMemberId, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.fatigue_details.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fatigue-detail", Vec::<String>::new());
    };
    if row.member_id == payload.new_member_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fatigue detail already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { fatigue_details: En1993FatigueDetailDelta::modification(payload.id.clone(), En1993FatigueDetailPatch { member_id: Some(payload.new_member_id.clone()), ..Default::default() }), ..Default::default() })
}
