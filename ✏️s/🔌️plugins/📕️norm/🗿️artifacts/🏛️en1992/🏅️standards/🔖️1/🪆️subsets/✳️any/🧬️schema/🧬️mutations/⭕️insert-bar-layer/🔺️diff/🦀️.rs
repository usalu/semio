//! ⭕ `insert-bar-layer` diff: inserts the bar layer row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_bar_layer::InsertBarLayer;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersLongitudinalRows, En1992MembersPatch, En1992MembersRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &InsertBarLayer, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.member_id), [payload.member_id.clone()]);
    };
    if r0.longitudinal.iter().any(|existing| existing.id == payload.bar_layer.id) {
        let key = payload.bar_layer.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Bar layer {key} already exists."), [key]);
    }
    if payload.index > r0.longitudinal.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, r0.longitudinal.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { longitudinal: Some(En1992MembersLongitudinalRows::insertion(payload.index, payload.bar_layer.clone())), ..Default::default() })), ..Default::default() })
}
