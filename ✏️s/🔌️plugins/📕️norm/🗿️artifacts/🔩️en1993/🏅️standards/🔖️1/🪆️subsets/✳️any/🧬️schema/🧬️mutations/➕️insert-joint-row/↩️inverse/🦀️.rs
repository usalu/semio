use crate::insert_joint_row::InsertJointRow;
use crate::remove_joint_row;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertJointRow, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_joint_row::RemoveJointRow> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.joint_rows.len());
    vec![Op::from(remove_joint_row::RemoveJointRow { index: at })]

    })())
}
