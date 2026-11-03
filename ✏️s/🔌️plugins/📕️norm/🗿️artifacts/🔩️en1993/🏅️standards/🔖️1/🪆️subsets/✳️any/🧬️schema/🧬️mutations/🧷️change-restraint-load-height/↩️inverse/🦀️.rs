//! ↩️ `change-restraint-load-height` inverse: restores the previous `load_height`.

use crate::change_restraint_load_height::ChangeRestraintLoadHeight;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRestraintLoadHeight, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_restraint_load_height::ChangeRestraintLoadHeight> {
    Ok(match base.restraints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRestraintLoadHeight { id: payload.id.clone(), new_load_height: row.load_height })],
        None => Vec::new(),
    })
}
