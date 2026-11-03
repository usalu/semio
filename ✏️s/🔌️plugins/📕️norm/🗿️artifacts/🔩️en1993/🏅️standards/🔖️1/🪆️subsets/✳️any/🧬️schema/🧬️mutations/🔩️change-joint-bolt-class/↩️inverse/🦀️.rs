//! ↩️ `change-joint-bolt-class` inverse: restores the previous `bolt_class`.

use crate::change_joint_bolt_class::ChangeJointBoltClass;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointBoltClass, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_bolt_class::ChangeJointBoltClass> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointBoltClass { id: payload.id.clone(), new_bolt_class: row.bolt_class.clone() })],
        None => Vec::new(),
    })
}
