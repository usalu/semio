use crate::insert_hollow_joint::InsertHollowJoint;
use crate::remove_hollow_joint;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertHollowJoint, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_hollow_joint::RemoveHollowJoint> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.hollow_joints.len());
    vec![Op::from(remove_hollow_joint::RemoveHollowJoint { index: at })]

    })())
}
