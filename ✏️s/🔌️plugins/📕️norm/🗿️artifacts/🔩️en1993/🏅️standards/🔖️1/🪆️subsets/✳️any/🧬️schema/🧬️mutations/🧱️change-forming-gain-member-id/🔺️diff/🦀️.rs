//! 🔺️ `change-forming-gain-member-id` diff: patches `member_id` of the addressed forming_gain block row.

use crate::change_forming_gain_member_id::ChangeFormingGainMemberId;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FormingGainDelta, En1993FormingGainPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFormingGainMemberId, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.forming_gains.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "forming-gain", Vec::<String>::new());
    };
    if row.member_id == payload.new_member_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The forming_gain block already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { forming_gains: En1993FormingGainDelta::modification(payload.id.clone(), En1993FormingGainPatch { member_id: Some(payload.new_member_id.clone()), ..Default::default() }), ..Default::default() })
}
