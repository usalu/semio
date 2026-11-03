use crate::remove_joint_plate::RemoveJointPlate;
use crate::insert_joint_plate;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveJointPlate, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_joint_plate::InsertJointPlate> {
    Ok((|| {
    if payload.index >= base.joint_plates.len() { return Vec::new(); }
    vec![Op::from(insert_joint_plate::InsertJointPlate { index: Some(payload.index), joint_plate: base.joint_plates[payload.index].clone() })]

    })())
}
