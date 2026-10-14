//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use crate::update_cold_formed_inputs::UpdateColdFormedInputs;
use crate::remove_cold_formed_member;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &UpdateColdFormedInputs, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::update_cold_formed_inputs::UpdateColdFormedInputs> + From<crate::remove_cold_formed_member::RemoveColdFormedMember> {
    Ok((|| {
    if let Some(prior) = base.cold_formed_members.iter().find(|x| x.id == payload.cold_formed_member.id) {
        vec![Op::from(UpdateColdFormedInputs { cold_formed_member: prior.clone() })]
    } else {
        vec![Op::from(remove_cold_formed_member::RemoveColdFormedMember { index: base.cold_formed_members.len() })]
    }

    })())
}
