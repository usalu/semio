//! ↩️ `update-stainless-inputs` — undo restores BASE's stainless inputs.

use super::UpdateStainlessInputs;
use crate::artifacts::en1993::{En1993Mutation, En1993Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &UpdateStainlessInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    vec![En1993Mutation::UpdateStainlessInputs(UpdateStainlessInputs { new_stainless_m_ed_knm: base.stainless_m_ed_knm, new_stainless_w_pl_mm3: base.stainless_w_pl_mm3, new_stainless_f_y_mpa: base.stainless_f_y_mpa })]
}
//#endregion 🔖️Inverse
