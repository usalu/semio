//! ↩️ `change-joint-xm` inverse: restores the previous `x_m`.

use crate::change_joint_x_m::ChangeJointXM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeJointXM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_x_m::ChangeJointXM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeJointXM, base: &En1998Snapshot) -> Option<ChangeJointXM> {
    let r0 = base.joints.iter().find(|row| row.id == payload.id)?;
    Some(ChangeJointXM { id: payload.id.clone(), new_x_m: r0.x_m })
}
