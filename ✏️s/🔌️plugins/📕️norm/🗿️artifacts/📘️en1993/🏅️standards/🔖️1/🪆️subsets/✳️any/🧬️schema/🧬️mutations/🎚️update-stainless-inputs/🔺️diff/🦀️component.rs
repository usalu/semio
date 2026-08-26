//! 🔺️ `update-stainless-inputs` — sparse diff construction.

use super::mutation::UpdateStainlessInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateStainlessInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_stainless_m_ed_knm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Stainless m ed knm must be a finite number, got {}.", payload.new_stainless_m_ed_knm), Vec::<String>::new());
    }
    if !payload.new_stainless_w_pl_mm3.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Stainless w pl mm3 must be a finite number, got {}.", payload.new_stainless_w_pl_mm3), Vec::<String>::new());
    }
    if !payload.new_stainless_f_y_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Stainless f y mpa must be a finite number, got {}.", payload.new_stainless_f_y_mpa), Vec::<String>::new());
    }
    if base.stainless_m_ed_knm == payload.new_stainless_m_ed_knm && base.stainless_w_pl_mm3 == payload.new_stainless_w_pl_mm3 && base.stainless_f_y_mpa == payload.new_stainless_f_y_mpa {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "This facet already has these values.");
    }
    protocol::MutationOutcome::new(En1993Diff { stainless_m_ed_knm: Some(payload.new_stainless_m_ed_knm), stainless_w_pl_mm3: Some(payload.new_stainless_w_pl_mm3), stainless_f_y_mpa: Some(payload.new_stainless_f_y_mpa), ..Default::default() })
}
//#endregion 🔖️Diff
