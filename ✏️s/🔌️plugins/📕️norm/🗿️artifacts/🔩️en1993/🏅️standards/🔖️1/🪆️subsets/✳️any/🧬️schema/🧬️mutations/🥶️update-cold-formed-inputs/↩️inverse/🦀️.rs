//! ↩️ `update-cold-formed-inputs` — undo restores BASE's cold formed inputs.

use super::UpdateColdFormedInputs;
use crate::artifacts::en1993::{En1993Mutation, En1993Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &UpdateColdFormedInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    vec![En1993Mutation::UpdateColdFormedInputs(UpdateColdFormedInputs {
        new_cf_b_bar_mm: base.cf_b_bar_mm,
        new_cf_t_mm: base.cf_t_mm,
        new_cf_k_sigma: base.cf_k_sigma,
        new_cf_psi: base.cf_psi,
        new_cf_n_ed_kn: base.cf_n_ed_kn,
        new_cf_gross_resistance_kn: base.cf_gross_resistance_kn,
    })]
}
//#endregion 🔖️Inverse
