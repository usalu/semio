//! ↩️ `update-plated-inputs` — undo restores BASE's plated inputs.

use super::mutation::UpdatePlatedInputs;
use crate::artifacts::en1993::{En1993Mutation, En1993Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &UpdatePlatedInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    vec![En1993Mutation::UpdatePlatedInputs(UpdatePlatedInputs { new_plated_lambda_p: base.plated_lambda_p, new_plated_sigma_ed_mpa: base.plated_sigma_ed_mpa })]
}
//#endregion 🔖️Inverse
