//! ↩️ `change-restraint-intermediate-position` inverse: restores the previous `intermediate_position`.

use crate::change_restraint_intermediate_position::ChangeRestraintIntermediatePosition;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRestraintIntermediatePosition, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_restraint_intermediate_position::ChangeRestraintIntermediatePosition> {
    Ok(match base.restraints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRestraintIntermediatePosition { id: payload.id.clone(), new_intermediate_position: row.intermediate_position })],
        None => Vec::new(),
    })
}
