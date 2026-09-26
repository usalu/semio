use super::InsertJoint;
use crate::mutations::{remove_joint, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &InsertJoint, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    let at = payload.index.min(base.joints.len());
    vec![En1993Mutation::RemoveJoint(remove_joint::RemoveJoint { index: at })]
}
