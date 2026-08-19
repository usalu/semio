//! ↩️ `update-tower-inputs` — undo restores BASE's tower inputs.

use super::mutation::UpdateTowerInputs;
use crate::artifacts::en1993::{En1993Mutation, En1993Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &UpdateTowerInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    vec![En1993Mutation::UpdateTowerInputs(UpdateTowerInputs { new_tower_wind_factor: base.tower_wind_factor, new_tower_n_ed_kn: base.tower_n_ed_kn })]
}
//#endregion 🔖️Inverse
