//! ✏️ `change-member-min-dimension-m` diff: patches `min_dimension_m` of the addressed member row.

use crate::change_member_min_dimension_m::ChangeMemberMinDimensionM;
use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff, En1998MemberDelta, En1998MemberPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeMemberMinDimensionM, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_min_dimension_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.buildings.iter().find(|row| row.id == payload.building_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("building {} does not exist.", payload.building_id), [payload.building_id.clone()]);
    };
    let Some(r1) = r0.members.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("member {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r1.min_dimension_m == payload.new_min_dimension_m {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&payload.building_id, En1998BuildingPatch { members: En1998MemberDelta::modification(&payload.id, En1998MemberPatch { min_dimension_m: Some(payload.new_min_dimension_m), ..Default::default() }), ..Default::default() }), ..Default::default() })
}
