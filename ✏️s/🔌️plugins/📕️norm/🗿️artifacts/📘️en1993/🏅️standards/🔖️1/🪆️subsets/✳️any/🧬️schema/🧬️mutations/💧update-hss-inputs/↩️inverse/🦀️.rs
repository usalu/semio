//! ↩️ `update-hss-inputs` — undo restores BASE's hss inputs.

use super::UpdateHssInputs;
use crate::artifacts::en1993::{En1993Mutation, En1993Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &UpdateHssInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    vec![En1993Mutation::UpdateHssInputs(UpdateHssInputs { new_hss_w_el_mm3: base.hss_w_el_mm3, new_hss_f_y_mpa: base.hss_f_y_mpa, new_hss_section_class: base.hss_section_class, new_hss_m_ed_knm: base.hss_m_ed_knm })]
}
//#endregion 🔖️Inverse
