//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use super::UpdateBoltInputs;
use crate::mutations::remove_joint;
use crate::{En1993Mutation, En1993Snapshot};
pub fn inverse(payload: &UpdateBoltInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    if let Some(prior) = base.joints.iter().find(|x| x.id == payload.joint.id) {
        vec![En1993Mutation::UpdateBoltInputs(UpdateBoltInputs { joint: prior.clone() })]
    } else {
        vec![En1993Mutation::RemoveJoint(remove_joint::RemoveJoint { index: base.joints.len() })]
    }
}
