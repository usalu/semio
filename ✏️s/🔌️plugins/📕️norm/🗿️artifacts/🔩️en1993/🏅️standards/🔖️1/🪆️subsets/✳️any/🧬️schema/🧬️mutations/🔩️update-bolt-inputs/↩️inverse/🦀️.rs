//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use crate::update_bolt_inputs::UpdateBoltInputs;
use crate::remove_joint;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &UpdateBoltInputs, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::update_bolt_inputs::UpdateBoltInputs> + From<crate::remove_joint::RemoveJoint> {
    Ok((|| {
    if let Some(prior) = base.joints.iter().find(|x| x.id == payload.joint.id) {
        vec![Op::from(UpdateBoltInputs { joint: prior.clone() })]
    } else {
        vec![Op::from(remove_joint::RemoveJoint { index: base.joints.len() })]
    }

    })())
}
