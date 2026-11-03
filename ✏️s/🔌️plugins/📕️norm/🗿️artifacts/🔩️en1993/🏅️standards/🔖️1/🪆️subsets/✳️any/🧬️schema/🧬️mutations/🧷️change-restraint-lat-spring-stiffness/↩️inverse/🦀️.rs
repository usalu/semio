//! ↩️ `change-restraint-lat-spring-stiffness` inverse: restores the previous `lateral_spring_stiffness`.

use crate::change_restraint_lat_spring_stiffness::ChangeRestraintLatSpringStiffness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRestraintLatSpringStiffness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_restraint_lat_spring_stiffness::ChangeRestraintLatSpringStiffness> {
    Ok(match base.restraints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRestraintLatSpringStiffness { id: payload.id.clone(), new_lateral_spring_stiffness: row.lateral_spring_stiffness })],
        None => Vec::new(),
    })
}
