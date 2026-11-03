//! 🔺️ `change-fatigue-member-id` diff: patches `member_id` of the addressed fatigue detail row.

use crate::change_fatigue_member_id::ChangeFatigueMemberId;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999FatigueDetailsPatch, En1999FatigueDetailsRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeFatigueMemberId, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(r0) = base.fatigue_details.iter().find(|row| row.id == payload.fatigue_detail_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fatigue-detail", Vec::<String>::new());
    };
    if r0.member_id == payload.new_member_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fatigue detail already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { fatigue_details: Some(En1999FatigueDetailsRows::modification(&payload.fatigue_detail_id, En1999FatigueDetailsPatch { member_id: Some(payload.new_member_id.clone()), ..Default::default() })), ..Default::default() })
}
