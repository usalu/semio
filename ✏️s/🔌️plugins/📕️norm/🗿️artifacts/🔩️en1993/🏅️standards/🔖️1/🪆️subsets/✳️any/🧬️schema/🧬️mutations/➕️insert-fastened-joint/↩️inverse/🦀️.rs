use crate::insert_fastened_joint::InsertFastenedJoint;
use crate::remove_fastened_joint;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertFastenedJoint, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_fastened_joint::RemoveFastenedJoint> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.fastened_joints.len());
    vec![Op::from(remove_fastened_joint::RemoveFastenedJoint { index: at })]

    })())
}
