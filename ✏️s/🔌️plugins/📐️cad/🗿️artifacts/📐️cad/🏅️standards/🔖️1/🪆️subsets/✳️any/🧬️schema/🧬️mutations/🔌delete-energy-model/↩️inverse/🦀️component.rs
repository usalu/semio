//! ↩️ `delete-energy-model` — undo is `create-energy-model` with the escrowed handle captured from BASE;
//! empty (`Vec::new()`) when the slot was already absent (nothing to undo).

use super::mutation::DeleteEnergyModel;
use crate::artifacts::cad::mutations::{create_energy_model, CadMutation};
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &DeleteEnergyModel, base: &CadSnapshot) -> Vec<CadMutation> {
    match &base.energy_model {
        Some(existing) => vec![CadMutation::CreateEnergyModel(create_energy_model::mutation::CreateEnergyModel { child_id: existing.child_id.clone(), target: existing.target.to_uri() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
