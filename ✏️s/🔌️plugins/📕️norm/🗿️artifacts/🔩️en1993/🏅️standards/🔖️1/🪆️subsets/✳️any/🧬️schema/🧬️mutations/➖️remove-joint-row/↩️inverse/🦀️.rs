use crate::remove_joint_row::RemoveJointRow;
use crate::insert_joint_row;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveJointRow, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_joint_row::InsertJointRow> {
    Ok((|| {
    if payload.index >= base.joint_rows.len() { return Vec::new(); }
    vec![Op::from(insert_joint_row::InsertJointRow { index: Some(payload.index), joint_row: base.joint_rows[payload.index].clone() })]

    })())
}
