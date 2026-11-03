//! ↩️ `change-joint-concrete-area-a1` inverse: restores the previous `concrete_area_a1`.

use crate::change_joint_concrete_area_a1::ChangeJointConcreteAreaA1;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointConcreteAreaA1, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_concrete_area_a1::ChangeJointConcreteAreaA1> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointConcreteAreaA1 { id: payload.id.clone(), new_concrete_area_a1: row.concrete_area_a1 })],
        None => Vec::new(),
    })
}
