//! ↩️ Inverse for `change-joint-col-moment-rd-nm`.
use crate::change_joint_col_moment_rd_nm::ChangeJointColMomentRdNm;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeJointColMomentRdNm, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_col_moment_rd_nm::ChangeJointColMomentRdNm> {
    Ok(match base.joints.get(payload.index) {
        Some(row) => vec![Op::from(ChangeJointColMomentRdNm { index: payload.index, new_column_moment_rd_nm: row.column_moment_rd_nm })],
        None => Vec::new(),
    })
}
