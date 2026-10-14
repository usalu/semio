use crate::insert_joint::InsertJoint;
use crate::remove_joint;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertJoint, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_joint::RemoveJoint> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.joints.len());
    vec![Op::from(remove_joint::RemoveJoint { index: at })]

    })())
}
