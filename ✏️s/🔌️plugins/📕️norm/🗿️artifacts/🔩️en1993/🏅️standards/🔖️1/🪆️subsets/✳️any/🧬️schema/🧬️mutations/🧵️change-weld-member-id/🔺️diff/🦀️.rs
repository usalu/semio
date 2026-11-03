//! 🔺️ `change-weld-member-id` diff: patches `member_id` of the addressed weld group row.

use crate::change_weld_member_id::ChangeWeldMemberId;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993WeldGroupDelta, En1993WeldGroupPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeWeldMemberId, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.weld_groups.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "weld-group", Vec::<String>::new());
    };
    if row.member_id == payload.new_member_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The weld group already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { weld_groups: En1993WeldGroupDelta::modification(payload.id.clone(), En1993WeldGroupPatch { member_id: Some(payload.new_member_id.clone()), ..Default::default() }), ..Default::default() })
}
