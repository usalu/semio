//! 🔺️ `update-hss-inputs` — sparse diff construction.

use super::mutation::UpdateHssInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateHssInputs, _base: &En1993Snapshot) -> En1993Diff {
    En1993Diff {
        hss_w_el_mm3: Some(payload.new_hss_w_el_mm3),
        hss_f_y_mpa: Some(payload.new_hss_f_y_mpa),
        hss_section_class: Some(payload.new_hss_section_class),
        hss_m_ed_knm: Some(payload.new_hss_m_ed_knm),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
