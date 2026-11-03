//! ↩️ `change-joint-concrete-fcd` inverse: restores the previous `concrete_fcd`.

use crate::change_joint_concrete_fcd::ChangeJointConcreteFcd;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointConcreteFcd, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_concrete_fcd::ChangeJointConcreteFcd> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointConcreteFcd { id: payload.id.clone(), new_concrete_fcd: row.concrete_fcd })],
        None => Vec::new(),
    })
}
