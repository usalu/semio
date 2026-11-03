//! ➖️ `remove-member` diff: removes the addressed member row.

use crate::remove_member::RemoveMember;
use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff, En1998MemberDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemoveMember, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.buildings.iter().find(|row| row.id == payload.building_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("building {} does not exist.", payload.building_id), [payload.building_id.clone()]);
    };
    let Some(index) = r0.members.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("member {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&payload.building_id, En1998BuildingPatch { members: En1998MemberDelta::removal(&r0.members, index), ..Default::default() }), ..Default::default() })
}
