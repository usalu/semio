//! ↩️ `change-joint-moment-demand` inverse: restores the previous `moment_demand`.

use crate::change_joint_moment_demand::ChangeJointMomentDemand;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointMomentDemand, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_moment_demand::ChangeJointMomentDemand> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointMomentDemand { id: payload.id.clone(), new_moment_demand: row.moment_demand })],
        None => Vec::new(),
    })
}
