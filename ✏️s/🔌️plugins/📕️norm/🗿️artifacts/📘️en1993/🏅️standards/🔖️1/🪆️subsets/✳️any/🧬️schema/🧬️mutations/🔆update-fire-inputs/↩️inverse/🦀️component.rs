//! ↩️ `update-fire-inputs` — undo restores BASE's fire inputs.

use super::mutation::UpdateFireInputs;
use crate::artifacts::en1993::{En1993Mutation, En1993Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &UpdateFireInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    vec![En1993Mutation::UpdateFireInputs(UpdateFireInputs {
        new_fire_thickness_mm: base.fire_thickness_mm,
        new_fire_rating: base.fire_rating.clone(),
        new_fire_massivity: base.fire_massivity,
        new_fire_mu_0: base.fire_mu_0,
        new_fire_design_temperature_c: base.fire_design_temperature_c,
    })]
}
//#endregion 🔖️Inverse
