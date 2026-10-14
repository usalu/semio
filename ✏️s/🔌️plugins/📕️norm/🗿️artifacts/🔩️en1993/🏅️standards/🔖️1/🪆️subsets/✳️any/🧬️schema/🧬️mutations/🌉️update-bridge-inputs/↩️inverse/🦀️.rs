//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use crate::update_bridge_inputs::UpdateBridgeInputs;
use crate::remove_bridge_fatigue;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &UpdateBridgeInputs, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::update_bridge_inputs::UpdateBridgeInputs> + From<crate::remove_bridge_fatigue::RemoveBridgeFatigue> {
    Ok((|| {
    if let Some(prior) = base.bridge_fatigue.iter().find(|x| x.id == payload.bridge_fatigue_item.id) {
        vec![Op::from(UpdateBridgeInputs { bridge_fatigue_item: prior.clone() })]
    } else {
        vec![Op::from(remove_bridge_fatigue::RemoveBridgeFatigue { index: base.bridge_fatigue.len() })]
    }

    })())
}
