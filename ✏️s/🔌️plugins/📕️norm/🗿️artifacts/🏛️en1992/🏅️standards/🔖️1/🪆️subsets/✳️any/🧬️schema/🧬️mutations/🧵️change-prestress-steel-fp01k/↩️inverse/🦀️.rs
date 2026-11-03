//! ↩️ `change-prestress-steel-fp01k` inverse: restores the previous `f_p01k`.

use crate::change_prestress_steel_f_p01k::ChangePrestressSteelFP01k;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangePrestressSteelFP01k, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_prestress_steel_f_p01k::ChangePrestressSteelFP01k> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePrestressSteelFP01k, base: &En1992Snapshot) -> Option<ChangePrestressSteelFP01k> {
    let r0 = base.prestress_steels.iter().find(|row| row.id == payload.grade_id)?;
    Some(ChangePrestressSteelFP01k { grade_id: payload.grade_id.clone(), new_f_p01k: r0.f_p0_1k })
}
