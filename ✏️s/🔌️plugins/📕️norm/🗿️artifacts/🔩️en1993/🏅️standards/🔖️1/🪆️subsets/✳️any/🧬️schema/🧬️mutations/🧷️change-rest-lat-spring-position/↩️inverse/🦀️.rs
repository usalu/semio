//! ↩️ `change-rest-lat-spring-position` inverse: restores the previous `lateral_spring_position`.

use crate::change_rest_lat_spring_position::ChangeRestLatSpringPosition;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRestLatSpringPosition, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_rest_lat_spring_position::ChangeRestLatSpringPosition> {
    Ok(match base.restraints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRestLatSpringPosition { id: payload.id.clone(), new_lateral_spring_position: row.lateral_spring_position })],
        None => Vec::new(),
    })
}
