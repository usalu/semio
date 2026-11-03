//! ➕️ `insert-member` diff: inserts the member row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_member::InsertMember;
use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff, En1998MemberDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &InsertMember, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.buildings.iter().find(|row| row.id == payload.building_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("building {} does not exist.", payload.building_id), [payload.building_id.clone()]);
    };
    if r0.members.iter().any(|existing| existing.id == payload.member.id) {
        let key = payload.member.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("member {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(r0.members.len());
    let outcome = protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&payload.building_id, En1998BuildingPatch { members: En1998MemberDelta::insertion(index, payload.member.clone()), ..Default::default() }), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the member list; inserted at {index}.", payload.index.unwrap_or(index)))
}
