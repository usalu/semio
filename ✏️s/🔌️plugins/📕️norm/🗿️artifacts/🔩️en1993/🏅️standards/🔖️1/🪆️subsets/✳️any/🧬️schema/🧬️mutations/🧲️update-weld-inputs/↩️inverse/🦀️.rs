//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use crate::update_weld_inputs::UpdateWeldInputs;
use crate::remove_member_action;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &UpdateWeldInputs, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::update_weld_inputs::UpdateWeldInputs> + From<crate::remove_member_action::RemoveMemberAction> {
    Ok((|| {
    if let Some(prior) = base.member_actions.iter().find(|x| x.id == payload.member_action.id) {
        vec![Op::from(UpdateWeldInputs { member_action: prior.clone() })]
    } else {
        vec![Op::from(remove_member_action::RemoveMemberAction { index: base.member_actions.len() })]
    }

    })())
}
