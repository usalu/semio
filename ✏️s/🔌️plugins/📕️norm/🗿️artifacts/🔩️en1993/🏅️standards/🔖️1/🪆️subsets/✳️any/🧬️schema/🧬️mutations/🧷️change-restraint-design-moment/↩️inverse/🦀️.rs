//! ↩️ `change-restraint-design-moment` inverse: restores the previous `design_moment`.

use crate::change_restraint_design_moment::ChangeRestraintDesignMoment;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRestraintDesignMoment, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_restraint_design_moment::ChangeRestraintDesignMoment> {
    Ok(match base.restraints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRestraintDesignMoment { id: payload.id.clone(), new_design_moment: row.design_moment })],
        None => Vec::new(),
    })
}
