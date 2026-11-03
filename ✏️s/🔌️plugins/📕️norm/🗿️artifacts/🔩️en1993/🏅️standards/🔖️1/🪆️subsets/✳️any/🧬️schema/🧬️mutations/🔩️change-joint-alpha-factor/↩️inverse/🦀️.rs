//! ↩️ `change-joint-alpha-factor` inverse: restores the previous `alpha_factor`.

use crate::change_joint_alpha_factor::ChangeJointAlphaFactor;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointAlphaFactor, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_alpha_factor::ChangeJointAlphaFactor> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointAlphaFactor { id: payload.id.clone(), new_alpha_factor: row.alpha_factor })],
        None => Vec::new(),
    })
}
