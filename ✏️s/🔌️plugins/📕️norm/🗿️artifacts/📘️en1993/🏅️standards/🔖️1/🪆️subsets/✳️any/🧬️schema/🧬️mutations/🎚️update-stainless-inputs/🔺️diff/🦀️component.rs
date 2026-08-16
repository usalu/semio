//! 🔺️ `update-stainless-inputs` — sparse diff construction.

use super::mutation::UpdateStainlessInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateStainlessInputs, _base: &En1993Snapshot) -> En1993Diff {
    En1993Diff { stainless_m_ed_knm: Some(payload.new_stainless_m_ed_knm), stainless_w_pl_mm3: Some(payload.new_stainless_w_pl_mm3), stainless_f_y_mpa: Some(payload.new_stainless_f_y_mpa), ..Default::default() }
}
//#endregion 🔖️Diff
