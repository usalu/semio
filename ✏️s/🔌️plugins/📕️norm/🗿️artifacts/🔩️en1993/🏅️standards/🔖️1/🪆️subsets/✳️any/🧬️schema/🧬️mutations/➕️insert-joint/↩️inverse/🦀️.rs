use super::InsertJoint;
use crate::mutations::{remove_joint, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &InsertJoint, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.joints.len());
    vec![En1993Mutation::RemoveJoint(remove_joint::RemoveJoint { index: at })]

    })())
}
