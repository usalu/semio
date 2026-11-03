//! ↩️ `change-joint-column-moment-rd-nm` inverse: restores the previous `column_moment_rd_nm`.

use crate::change_joint_column_moment_rd_nm::ChangeJointColumnMomentRdNm;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeJointColumnMomentRdNm, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_column_moment_rd_nm::ChangeJointColumnMomentRdNm> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeJointColumnMomentRdNm, base: &En1998Snapshot) -> Option<ChangeJointColumnMomentRdNm> {
    let r0 = base.joints.iter().find(|row| row.id == payload.id)?;
    Some(ChangeJointColumnMomentRdNm { id: payload.id.clone(), new_column_moment_rd_nm: r0.column_moment_rd_nm })
}
