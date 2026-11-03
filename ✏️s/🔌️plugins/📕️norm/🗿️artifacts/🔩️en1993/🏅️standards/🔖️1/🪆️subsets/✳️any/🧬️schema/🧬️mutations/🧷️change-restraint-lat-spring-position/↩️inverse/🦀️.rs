//! ↩️ `change-restraint-lat-spring-position` inverse: restores the previous `lateral_spring_position`.

use crate::change_restraint_lat_spring_position::ChangeRestraintLatSpringPosition;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRestraintLatSpringPosition, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_restraint_lat_spring_position::ChangeRestraintLatSpringPosition> {
    Ok(match base.restraints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRestraintLatSpringPosition { id: payload.id.clone(), new_lateral_spring_position: row.lateral_spring_position })],
        None => Vec::new(),
    })
}
