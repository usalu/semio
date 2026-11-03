//! ↩️ `change-prestress-steel-name` inverse: restores the previous `name`.

use crate::change_prestress_steel_name::ChangePrestressSteelName;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangePrestressSteelName, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_prestress_steel_name::ChangePrestressSteelName> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePrestressSteelName, base: &En1992Snapshot) -> Option<ChangePrestressSteelName> {
    let r0 = base.prestress_steels.iter().find(|row| row.id == payload.grade_id)?;
    Some(ChangePrestressSteelName { grade_id: payload.grade_id.clone(), new_name: r0.name.clone() })
}
