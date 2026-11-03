//! ↩️ `change-bar-layer-bond-condition` inverse: restores the previous `bond_condition`.

use crate::change_bar_layer_bond_condition::ChangeBarLayerBondCondition;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeBarLayerBondCondition, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bar_layer_bond_condition::ChangeBarLayerBondCondition> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBarLayerBondCondition, base: &En1992Snapshot) -> Option<ChangeBarLayerBondCondition> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let r1 = r0.longitudinal.iter().find(|row| row.id == payload.layer_id)?;
    Some(ChangeBarLayerBondCondition { member_id: payload.member_id.clone(), layer_id: payload.layer_id.clone(), new_bond_condition: r1.bond_condition })
}
