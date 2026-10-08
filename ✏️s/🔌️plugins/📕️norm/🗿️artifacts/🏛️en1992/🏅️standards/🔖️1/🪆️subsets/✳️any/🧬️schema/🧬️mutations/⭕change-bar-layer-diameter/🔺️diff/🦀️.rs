use crate::diff::{En1992Diff, En1992MembersRows, En1992MembersPatch, En1992MembersLongitudinalRows, En1992MembersLongitudinalPatch};
use super::ChangeBarLayerDiameter;
use crate::En1992Snapshot;

pub fn diff(payload: &ChangeBarLayerDiameter, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(m) = base.members.iter().find(|m| m.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} not found.", payload.member_id), Vec::<String>::new());
    };
    let Some(layer) = m.longitudinal.iter().find(|l| l.id == payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer {} not found.", payload.layer_id), Vec::<String>::new());
    };
    if (layer.diameter - payload.new_diameter).abs() < f64::EPSILON {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1992Diff {
        members: Some(En1992MembersRows {
            modified: vec![En1992MembersPatch {
                id: payload.member_id.clone(),
                longitudinal: Some(En1992MembersLongitudinalRows { modified: vec![En1992MembersLongitudinalPatch { id: payload.layer_id.clone(), diameter: Some(payload.new_diameter), ..Default::default() }] }),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
