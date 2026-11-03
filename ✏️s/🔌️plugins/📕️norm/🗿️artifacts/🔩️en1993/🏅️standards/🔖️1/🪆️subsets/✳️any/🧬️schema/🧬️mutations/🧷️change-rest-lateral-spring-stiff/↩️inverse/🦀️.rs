//! ↩️ `change-rest-lateral-spring-stiff` inverse: restores the previous `lateral_spring_stiffness`.

use crate::change_rest_lateral_spring_stiff::ChangeRestLateralSpringStiff;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRestLateralSpringStiff, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_rest_lateral_spring_stiff::ChangeRestLateralSpringStiff> {
    Ok(match base.restraints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRestLateralSpringStiff { id: payload.id.clone(), new_lateral_spring_stiffness: row.lateral_spring_stiffness })],
        None => Vec::new(),
    })
}
