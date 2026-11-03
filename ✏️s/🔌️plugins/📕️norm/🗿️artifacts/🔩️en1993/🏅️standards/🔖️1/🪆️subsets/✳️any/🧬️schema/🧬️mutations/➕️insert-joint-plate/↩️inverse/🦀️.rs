use crate::insert_joint_plate::InsertJointPlate;
use crate::remove_joint_plate;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertJointPlate, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_joint_plate::RemoveJointPlate> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.joint_plates.len());
    vec![Op::from(remove_joint_plate::RemoveJointPlate { index: at })]

    })())
}
