//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use crate::update_tower_inputs::UpdateTowerInputs;
use crate::remove_tower_leg;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &UpdateTowerInputs, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::update_tower_inputs::UpdateTowerInputs> + From<crate::remove_tower_leg::RemoveTowerLeg> {
    Ok((|| {
    if let Some(prior) = base.tower_legs.iter().find(|x| x.id == payload.tower_leg.id) {
        vec![Op::from(UpdateTowerInputs { tower_leg: prior.clone() })]
    } else {
        vec![Op::from(remove_tower_leg::RemoveTowerLeg { index: base.tower_legs.len() })]
    }

    })())
}
