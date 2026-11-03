//! ↩️ `change-prestress-steel-f-pk` inverse: restores the previous `f_pk`.

use crate::change_prestress_steel_f_pk::ChangePrestressSteelFPk;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangePrestressSteelFPk, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_prestress_steel_f_pk::ChangePrestressSteelFPk> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePrestressSteelFPk, base: &En1992Snapshot) -> Option<ChangePrestressSteelFPk> {
    let r0 = base.prestress_steels.iter().find(|row| row.id == payload.grade_id)?;
    Some(ChangePrestressSteelFPk { grade_id: payload.grade_id.clone(), new_f_pk: r0.f_pk })
}
