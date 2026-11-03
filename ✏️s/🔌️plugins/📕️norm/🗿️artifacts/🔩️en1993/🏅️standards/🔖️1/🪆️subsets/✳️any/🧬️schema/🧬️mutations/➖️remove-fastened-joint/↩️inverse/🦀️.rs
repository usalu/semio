use crate::remove_fastened_joint::RemoveFastenedJoint;
use crate::insert_fastened_joint;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveFastenedJoint, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_fastened_joint::InsertFastenedJoint> {
    Ok((|| {
    if payload.index >= base.fastened_joints.len() { return Vec::new(); }
    vec![Op::from(insert_fastened_joint::InsertFastenedJoint { index: Some(payload.index), fastened_joint: base.fastened_joints[payload.index].clone() })]

    })())
}
