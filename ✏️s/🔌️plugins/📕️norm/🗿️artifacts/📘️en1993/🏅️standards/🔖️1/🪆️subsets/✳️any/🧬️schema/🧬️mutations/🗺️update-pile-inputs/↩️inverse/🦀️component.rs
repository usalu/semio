//! ↩️ `update-pile-inputs` — undo restores BASE's pile inputs.

use super::mutation::UpdatePileInputs;
use crate::artifacts::en1993::{En1993Mutation, En1993Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &UpdatePileInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    vec![En1993Mutation::UpdatePileInputs(UpdatePileInputs { new_pile_sigma_mpa: base.pile_sigma_mpa, new_pile_k_red: base.pile_k_red, new_pile_n_ed_kn: base.pile_n_ed_kn })]
}
//#endregion 🔖️Inverse
