//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use crate::update_crane_inputs::UpdateCraneInputs;
use crate::remove_crane_runway;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &UpdateCraneInputs, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::update_crane_inputs::UpdateCraneInputs> + From<crate::remove_crane_runway::RemoveCraneRunway> {
    Ok((|| {
    if let Some(prior) = base.crane_runways.iter().find(|x| x.id == payload.crane_runway.id) {
        vec![Op::from(UpdateCraneInputs { crane_runway: prior.clone() })]
    } else {
        vec![Op::from(remove_crane_runway::RemoveCraneRunway { index: base.crane_runways.len() })]
    }

    })())
}
