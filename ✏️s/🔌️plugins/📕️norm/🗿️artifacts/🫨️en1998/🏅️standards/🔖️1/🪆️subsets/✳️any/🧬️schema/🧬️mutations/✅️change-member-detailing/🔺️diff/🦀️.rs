//! ✅️ `change-member-detailing` diff — patches the one field of the nested row at the indexes; a missing building or member is a `mutation.target-missing`.

use super::ChangeMemberDetailing;
use crate::diff::En1998RowEdit as _;
use crate::diff::{En1998BuildingEdit, En1998BuildingPatch, En1998Diff, En1998MemberEdit, En1998MemberPatch};
use crate::En1998Snapshot;

pub fn diff(payload: &ChangeMemberDetailing, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(building) = base.buildings.get(payload.building_index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "building", Vec::<String>::new());
    };
    let Some(row) = building.members.get(payload.member_index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "member", Vec::<String>::new());
    };
    let nested = En1998MemberEdit::patch(payload.member_index, row.id.clone(), En1998MemberPatch { detailing_compatible_with_q: Some(payload.new_detailing_compatible_with_q), ..Default::default() });
    let patch = En1998BuildingPatch { members: vec![nested], ..Default::default() };
    protocol::MutationOutcome::new(En1998Diff { buildings: vec![En1998BuildingEdit::patch(payload.building_index, building.id.clone(), patch)], ..Default::default() })
}
