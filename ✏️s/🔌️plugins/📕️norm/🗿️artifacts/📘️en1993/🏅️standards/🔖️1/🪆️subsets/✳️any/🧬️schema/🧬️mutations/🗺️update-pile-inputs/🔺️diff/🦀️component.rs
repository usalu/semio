//! 🔺️ `update-pile-inputs` — sparse diff construction.

use super::mutation::UpdatePileInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdatePileInputs, _base: &En1993Snapshot) -> En1993Diff {
    En1993Diff {
        pile_sigma_mpa: Some(payload.new_pile_sigma_mpa),
        pile_k_red: Some(payload.new_pile_k_red),
        pile_n_ed_kn: Some(payload.new_pile_n_ed_kn),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
