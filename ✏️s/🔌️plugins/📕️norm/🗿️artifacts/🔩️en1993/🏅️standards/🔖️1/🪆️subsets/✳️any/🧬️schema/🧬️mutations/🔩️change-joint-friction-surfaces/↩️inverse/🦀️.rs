//! ↩️ `change-joint-friction-surfaces` inverse: restores the previous `friction_surfaces`.

use crate::change_joint_friction_surfaces::ChangeJointFrictionSurfaces;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointFrictionSurfaces, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_friction_surfaces::ChangeJointFrictionSurfaces> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointFrictionSurfaces { id: payload.id.clone(), new_friction_surfaces: row.friction_surfaces })],
        None => Vec::new(),
    })
}
