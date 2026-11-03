//! 🧬 `insert-tendon-point` diff: inserts the tendon point row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_tendon_point::InsertTendonPoint;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersPatch, En1992MembersRows, En1992MembersTendonPointsRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &InsertTendonPoint, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.member_id), [payload.member_id.clone()]);
    };
    if r0.tendon_points.iter().any(|existing| existing.id == payload.tendon_point.id) {
        let key = payload.tendon_point.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Tendon point {key} already exists."), [key]);
    }
    if payload.index > r0.tendon_points.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, r0.tendon_points.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { tendon_points: Some(En1992MembersTendonPointsRows::insertion(payload.index, payload.tendon_point.clone())), ..Default::default() })), ..Default::default() })
}
