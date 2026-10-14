//! ↩️ `delete-energy-model` — undo is `create-energy-model` with the escrowed handle captured from BASE;
//! empty (`Vec::new()`) when the slot was already absent (nothing to undo).

use super::DeleteEnergyModel;
use crate::create_energy_model;
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::create_energy_model::CreateEnergyModel>>(_payload: &DeleteEnergyModel, base: &CadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match &base.energy_model {
        Some(existing) => vec![Op::from(create_energy_model::CreateEnergyModel { child_id: existing.child_id.clone(), target: existing.target.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
