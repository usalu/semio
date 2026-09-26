use super::ChangeBarLayerDiameter;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

pub fn inverse(payload: &ChangeBarLayerDiameter, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    let Some(m) = base.members.iter().find(|m| m.id == payload.member_id) else { return vec![]; };
    let Some(layer) = m.longitudinal.iter().find(|l| l.id == payload.layer_id) else { return vec![]; };
    vec![En1992Mutation::ChangeBarLayerDiameter(ChangeBarLayerDiameter { member_id: payload.member_id.clone(), layer_id: payload.layer_id.clone(), new_diameter: layer.diameter })]
}
