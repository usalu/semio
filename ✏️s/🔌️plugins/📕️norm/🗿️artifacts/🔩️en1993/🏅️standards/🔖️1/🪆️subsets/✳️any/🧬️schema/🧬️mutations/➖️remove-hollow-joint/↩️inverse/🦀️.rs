use crate::remove_hollow_joint::RemoveHollowJoint;
use crate::insert_hollow_joint;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveHollowJoint, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_hollow_joint::InsertHollowJoint> {
    Ok((|| {
    if payload.index >= base.hollow_joints.len() { return Vec::new(); }
    vec![Op::from(insert_hollow_joint::InsertHollowJoint { index: Some(payload.index), hollow_joint: base.hollow_joints[payload.index].clone() })]

    })())
}
