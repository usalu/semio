//! ↩️ `change-restraint-design-axial` inverse: restores the previous `design_axial`.

use crate::change_restraint_design_axial::ChangeRestraintDesignAxial;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRestraintDesignAxial, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_restraint_design_axial::ChangeRestraintDesignAxial> {
    Ok(match base.restraints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRestraintDesignAxial { id: payload.id.clone(), new_design_axial: row.design_axial })],
        None => Vec::new(),
    })
}
