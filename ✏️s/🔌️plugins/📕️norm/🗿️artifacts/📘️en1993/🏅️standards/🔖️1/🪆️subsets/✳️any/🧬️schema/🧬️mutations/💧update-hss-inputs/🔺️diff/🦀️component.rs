//! 🔺️ `update-hss-inputs` — sparse diff construction.

use super::mutation::UpdateHssInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateHssInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_hss_w_el_mm3.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Hss w el mm3 must be a finite number, got {}.", payload.new_hss_w_el_mm3), Vec::<String>::new());
    }
    if !payload.new_hss_f_y_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Hss f y mpa must be a finite number, got {}.", payload.new_hss_f_y_mpa), Vec::<String>::new());
    }
    if !payload.new_hss_m_ed_knm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Hss m ed knm must be a finite number, got {}.", payload.new_hss_m_ed_knm), Vec::<String>::new());
    }
    if base.hss_w_el_mm3 == payload.new_hss_w_el_mm3 && base.hss_f_y_mpa == payload.new_hss_f_y_mpa && base.hss_section_class == payload.new_hss_section_class && base.hss_m_ed_knm == payload.new_hss_m_ed_knm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "This facet already has these values.");
    }
    protocol::MutationOutcome::new(En1993Diff { hss_w_el_mm3: Some(payload.new_hss_w_el_mm3), hss_f_y_mpa: Some(payload.new_hss_f_y_mpa), hss_section_class: Some(payload.new_hss_section_class), hss_m_ed_knm: Some(payload.new_hss_m_ed_knm), ..Default::default() })
}
//#endregion 🔖️Diff
