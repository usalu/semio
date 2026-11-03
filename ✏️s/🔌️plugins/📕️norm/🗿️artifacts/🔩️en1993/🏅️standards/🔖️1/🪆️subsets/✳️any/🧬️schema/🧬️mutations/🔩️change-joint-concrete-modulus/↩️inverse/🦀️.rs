//! ↩️ `change-joint-concrete-modulus` inverse: restores the previous `concrete_modulus`.

use crate::change_joint_concrete_modulus::ChangeJointConcreteModulus;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointConcreteModulus, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_concrete_modulus::ChangeJointConcreteModulus> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointConcreteModulus { id: payload.id.clone(), new_concrete_modulus: row.concrete_modulus })],
        None => Vec::new(),
    })
}
