//! 🔺️ `update-cold-formed-inputs` — sparse diff construction.

use super::mutation::UpdateColdFormedInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateColdFormedInputs, _base: &En1993Snapshot) -> En1993Diff {
    En1993Diff {
        cf_b_bar_mm: Some(payload.new_cf_b_bar_mm),
        cf_t_mm: Some(payload.new_cf_t_mm),
        cf_k_sigma: Some(payload.new_cf_k_sigma),
        cf_psi: Some(payload.new_cf_psi),
        cf_n_ed_kn: Some(payload.new_cf_n_ed_kn),
        cf_gross_resistance_kn: Some(payload.new_cf_gross_resistance_kn),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
