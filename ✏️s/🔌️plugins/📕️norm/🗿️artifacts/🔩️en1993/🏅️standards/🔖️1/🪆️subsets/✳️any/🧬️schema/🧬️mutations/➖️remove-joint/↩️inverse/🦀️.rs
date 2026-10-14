use crate::remove_joint::RemoveJoint;
use crate::insert_joint;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveJoint, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_joint::InsertJoint> {
    Ok((|| {
    if payload.index >= base.joints.len() { return Vec::new(); }
    vec![Op::from(insert_joint::InsertJoint { index: Some(payload.index), joint: base.joints[payload.index].clone() })]

    })())
}
