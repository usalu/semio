//! ↩️ `create-energy-model` — undo restores whichever handle occupied `energy_model` BEFORE this create ran
//! (a real prior handle if the slot was occupied, or `delete-energy-model` if it was empty) — never a
//! bare "delete", since `create-energy-model` may have OVERWRITTEN an existing handle.

use super::mutation::CreateEnergyModel;
use crate::artifacts::cad::mutations::{delete_energy_model, CadMutation};
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &CreateEnergyModel, base: &CadSnapshot) -> Vec<CadMutation> {
    match &base.energy_model {
        Some(existing) => vec![CadMutation::CreateEnergyModel(CreateEnergyModel { child_id: existing.child_id.clone(), target: existing.target.to_uri() })],
        None => vec![CadMutation::DeleteEnergyModel(delete_energy_model::mutation::DeleteEnergyModel {})],
    }
}
//#endregion 🔖️Inverse
