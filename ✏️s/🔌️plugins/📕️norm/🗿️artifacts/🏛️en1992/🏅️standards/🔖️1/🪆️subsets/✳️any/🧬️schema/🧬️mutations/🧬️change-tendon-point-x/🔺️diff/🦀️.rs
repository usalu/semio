//! 🧬 `change-tendon-point-x` diff: patches `x` of the addressed tendon point row.

use crate::change_tendon_point_x::ChangeTendonPointX;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersPatch, En1992MembersRows, En1992MembersTendonPointsPatch, En1992MembersTendonPointsRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeTendonPointX, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_x.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.member_id), [payload.member_id.clone()]);
    };
    let Some(r1) = r0.tendon_points.iter().find(|row| row.id == payload.tendon_point_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tendon point {} does not exist.", payload.tendon_point_id), [payload.tendon_point_id.clone()]);
    };
    if r1.x == payload.new_x {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The tendon point already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { tendon_points: Some(En1992MembersTendonPointsRows::modification(&payload.tendon_point_id, En1992MembersTendonPointsPatch { x: Some(payload.new_x), ..Default::default() })), ..Default::default() })), ..Default::default() })
}
