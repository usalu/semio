//! ✏️ `change-member-material` diff: patches `material` of the addressed member row.

use crate::change_member_material::ChangeMemberMaterial;
use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff, En1998MemberDelta, En1998MemberPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeMemberMaterial, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.buildings.iter().find(|row| row.id == payload.building_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("building {} does not exist.", payload.building_id), [payload.building_id.clone()]);
    };
    let Some(r1) = r0.members.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("member {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r1.material == payload.new_material {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&payload.building_id, En1998BuildingPatch { members: En1998MemberDelta::modification(&payload.id, En1998MemberPatch { material: Some(payload.new_material), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
