use super::RemoveJoint;
use crate::mutations::{insert_joint, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &RemoveJoint, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    if payload.index >= base.joints.len() { return Vec::new(); }
    vec![En1993Mutation::InsertJoint(insert_joint::InsertJoint { index: payload.index, joint: base.joints[payload.index].clone() })]
}
