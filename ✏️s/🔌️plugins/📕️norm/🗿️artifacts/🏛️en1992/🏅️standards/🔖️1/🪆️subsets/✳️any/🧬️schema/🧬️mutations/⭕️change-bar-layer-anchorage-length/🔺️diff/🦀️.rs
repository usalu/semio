//! ⭕ `change-bar-layer-anchorage-length` diff: patches `anchorage_length` of the addressed bar layer row.

use crate::change_bar_layer_anchorage_length::ChangeBarLayerAnchorageLength;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersLongitudinalPatch, En1992MembersLongitudinalRows, En1992MembersPatch, En1992MembersRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeBarLayerAnchorageLength, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_anchorage_length.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.member_id), [payload.member_id.clone()]);
    };
    let Some(r1) = r0.longitudinal.iter().find(|row| row.id == payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Bar layer {} does not exist.", payload.layer_id), [payload.layer_id.clone()]);
    };
    if r1.anchorage_length == payload.new_anchorage_length {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The bar layer already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { longitudinal: Some(En1992MembersLongitudinalRows::modification(&payload.layer_id, En1992MembersLongitudinalPatch { anchorage_length: Some(payload.new_anchorage_length), ..Default::default() })), ..Default::default() })), ..Default::default() })
}
