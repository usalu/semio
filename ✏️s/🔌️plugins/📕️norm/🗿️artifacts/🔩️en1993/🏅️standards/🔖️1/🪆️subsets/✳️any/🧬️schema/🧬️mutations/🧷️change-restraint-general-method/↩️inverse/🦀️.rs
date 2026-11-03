//! ↩️ `change-restraint-general-method` inverse: restores the previous `general_method`.

use crate::change_restraint_general_method::ChangeRestraintGeneralMethod;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRestraintGeneralMethod, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_restraint_general_method::ChangeRestraintGeneralMethod> {
    Ok(match base.restraints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRestraintGeneralMethod { id: payload.id.clone(), new_general_method: row.general_method })],
        None => Vec::new(),
    })
}
