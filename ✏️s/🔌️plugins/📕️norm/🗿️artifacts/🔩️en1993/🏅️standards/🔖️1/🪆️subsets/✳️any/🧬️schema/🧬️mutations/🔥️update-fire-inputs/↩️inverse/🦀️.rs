//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use crate::update_fire_inputs::UpdateFireInputs;
use crate::remove_fire_exposure;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &UpdateFireInputs, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::update_fire_inputs::UpdateFireInputs> + From<crate::remove_fire_exposure::RemoveFireExposure> {
    Ok((|| {
    if let Some(prior) = base.fire_exposures.iter().find(|x| x.id == payload.fire_exposure.id) {
        vec![Op::from(UpdateFireInputs { fire_exposure: prior.clone() })]
    } else {
        vec![Op::from(remove_fire_exposure::RemoveFireExposure { index: base.fire_exposures.len() })]
    }

    })())
}
