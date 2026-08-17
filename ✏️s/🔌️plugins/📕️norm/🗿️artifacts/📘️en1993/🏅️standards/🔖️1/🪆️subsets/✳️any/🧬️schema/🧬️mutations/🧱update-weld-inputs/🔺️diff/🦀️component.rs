//! 🔺️ `update-weld-inputs` — sparse diff construction.

use super::mutation::UpdateWeldInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateWeldInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_weld_a_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Weld a mm must be a finite number, got {}.", payload.new_weld_a_mm), Vec::<String>::new());
    }
    if !payload.new_weld_l_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Weld l mm must be a finite number, got {}.", payload.new_weld_l_mm), Vec::<String>::new());
    }
    if !payload.new_weld_f_u_mpa.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Weld f u mpa must be a finite number, got {}.", payload.new_weld_f_u_mpa), Vec::<String>::new());
    }
    if !payload.new_weld_f_ed_kn.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Weld f ed kn must be a finite number, got {}.", payload.new_weld_f_ed_kn), Vec::<String>::new());
    }
    if base.weld_a_mm == payload.new_weld_a_mm && base.weld_l_mm == payload.new_weld_l_mm && base.weld_f_u_mpa == payload.new_weld_f_u_mpa && base.weld_steel_grade == payload.new_weld_steel_grade && base.weld_f_ed_kn == payload.new_weld_f_ed_kn {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "This facet already has these values.");
    }
    protocol::MutationOutcome::new(En1993Diff { weld_a_mm: Some(payload.new_weld_a_mm), weld_l_mm: Some(payload.new_weld_l_mm), weld_f_u_mpa: Some(payload.new_weld_f_u_mpa), weld_steel_grade: Some(payload.new_weld_steel_grade.clone()), weld_f_ed_kn: Some(payload.new_weld_f_ed_kn), ..Default::default() })
}
//#endregion 🔖️Diff
