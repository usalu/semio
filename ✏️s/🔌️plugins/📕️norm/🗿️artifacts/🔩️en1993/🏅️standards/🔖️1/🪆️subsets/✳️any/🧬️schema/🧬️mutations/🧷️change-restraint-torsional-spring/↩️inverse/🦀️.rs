//! ↩️ `change-restraint-torsional-spring` inverse: restores the previous `torsional_spring`.

use crate::change_restraint_torsional_spring::ChangeRestraintTorsionalSpring;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRestraintTorsionalSpring, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_restraint_torsional_spring::ChangeRestraintTorsionalSpring> {
    Ok(match base.restraints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRestraintTorsionalSpring { id: payload.id.clone(), new_torsional_spring: row.torsional_spring })],
        None => Vec::new(),
    })
}
