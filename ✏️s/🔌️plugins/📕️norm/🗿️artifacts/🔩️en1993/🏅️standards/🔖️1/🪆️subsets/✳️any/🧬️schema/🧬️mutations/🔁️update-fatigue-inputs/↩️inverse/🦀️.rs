//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use crate::update_fatigue_inputs::UpdateFatigueInputs;
use crate::remove_fatigue_detail;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &UpdateFatigueInputs, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::update_fatigue_inputs::UpdateFatigueInputs> + From<crate::remove_fatigue_detail::RemoveFatigueDetail> {
    Ok((|| {
    if let Some(prior) = base.fatigue_details.iter().find(|x| x.id == payload.fatigue_detail.id) {
        vec![Op::from(UpdateFatigueInputs { fatigue_detail: prior.clone() })]
    } else {
        vec![Op::from(remove_fatigue_detail::RemoveFatigueDetail { index: base.fatigue_details.len() })]
    }

    })())
}
