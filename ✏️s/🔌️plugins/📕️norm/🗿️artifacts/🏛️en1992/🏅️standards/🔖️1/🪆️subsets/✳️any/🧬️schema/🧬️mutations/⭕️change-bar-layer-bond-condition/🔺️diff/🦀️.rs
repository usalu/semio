//! ⭕ `change-bar-layer-bond-condition` diff: patches `bond_condition` of the addressed bar layer row.

use crate::change_bar_layer_bond_condition::ChangeBarLayerBondCondition;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersLongitudinalPatch, En1992MembersLongitudinalRows, En1992MembersPatch, En1992MembersRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeBarLayerBondCondition, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.member_id), [payload.member_id.clone()]);
    };
    let Some(r1) = r0.longitudinal.iter().find(|row| row.id == payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Bar layer {} does not exist.", payload.layer_id), [payload.layer_id.clone()]);
    };
    if r1.bond_condition == payload.new_bond_condition {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The bar layer already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { longitudinal: Some(En1992MembersLongitudinalRows::modification(&payload.layer_id, En1992MembersLongitudinalPatch { bond_condition: Some(payload.new_bond_condition), ..Default::default() })), ..Default::default() })), ..Default::default() })
}
