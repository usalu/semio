//! ↩️ `change-joint-slip-factor-ks` inverse: restores the previous `slip_factor_ks`.

use crate::change_joint_slip_factor_ks::ChangeJointSlipFactorKs;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointSlipFactorKs, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_slip_factor_ks::ChangeJointSlipFactorKs> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointSlipFactorKs { id: payload.id.clone(), new_slip_factor_ks: row.slip_factor_ks })],
        None => Vec::new(),
    })
}
