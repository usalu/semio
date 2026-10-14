//! ↩️ `create-energy-model` — undo restores whichever handle occupied `energy_model` BEFORE this create ran
//! (a real prior handle if the slot was occupied, or `delete-energy-model` if it was empty) — never a
//! bare "delete", since `create-energy-model` may have OVERWRITTEN an existing handle.

use super::CreateEnergyModel;
use crate::delete_energy_model;
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::create_energy_model::CreateEnergyModel> + From<crate::delete_energy_model::DeleteEnergyModel>>(_payload: &CreateEnergyModel, base: &CadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match &base.energy_model {
        Some(existing) => vec![Op::from(CreateEnergyModel { child_id: existing.child_id.clone(), target: existing.target.clone() })],
        None => vec![Op::from(delete_energy_model::DeleteEnergyModel {})],
    })
}
//#endregion 🔖️Inverse
