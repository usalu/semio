//! 🧬 `remove-tendon-point` diff: removes the addressed tendon point row.

use crate::remove_tendon_point::RemoveTendonPoint;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersPatch, En1992MembersRows, En1992MembersTendonPointsRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &RemoveTendonPoint, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.member_id), [payload.member_id.clone()]);
    };
    let index = payload.index;
    if index >= r0.tendon_points.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "tendon_points index out of range", [index.to_string()]);
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { tendon_points: Some(En1992MembersTendonPointsRows::removal(&r0.tendon_points, index)), ..Default::default() })), ..Default::default() })
}
