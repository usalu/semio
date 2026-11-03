//! ↩️ `change-cf-required-restraint-stiffness` inverse: restores the previous `required_restraint_stiffness`.

use crate::change_cf_required_restraint_stiffness::ChangeCfRequiredRestraintStiffness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCfRequiredRestraintStiffness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cf_required_restraint_stiffness::ChangeCfRequiredRestraintStiffness> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCfRequiredRestraintStiffness { id: payload.id.clone(), new_required_restraint_stiffness: row.required_restraint_stiffness })],
        None => Vec::new(),
    })
}
