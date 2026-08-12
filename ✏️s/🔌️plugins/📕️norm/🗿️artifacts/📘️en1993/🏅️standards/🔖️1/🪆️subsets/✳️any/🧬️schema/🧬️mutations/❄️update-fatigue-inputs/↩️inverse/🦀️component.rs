//! ↩️ `update-fatigue-inputs` — undo restores BASE's fatigue inputs.

use super::mutation::UpdateFatigueInputs;
use crate::artifacts::en1993::{En1993Mutation, En1993Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &UpdateFatigueInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    vec![En1993Mutation::UpdateFatigueInputs(UpdateFatigueInputs {
        new_delta_sigma_mpa: base.delta_sigma_mpa,
        new_fatigue_category: base.fatigue_category,
        new_fatigue_method: base.fatigue_method.clone(),
    })]
}
//#endregion 🔖️Inverse
