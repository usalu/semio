//! ↩️ `change-joint-ym` inverse: restores the previous `y_m`.

use crate::change_joint_y_m::ChangeJointYM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeJointYM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_y_m::ChangeJointYM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeJointYM, base: &En1998Snapshot) -> Option<ChangeJointYM> {
    let r0 = base.joints.iter().find(|row| row.id == payload.id)?;
    Some(ChangeJointYM { id: payload.id.clone(), new_y_m: r0.y_m })
}
