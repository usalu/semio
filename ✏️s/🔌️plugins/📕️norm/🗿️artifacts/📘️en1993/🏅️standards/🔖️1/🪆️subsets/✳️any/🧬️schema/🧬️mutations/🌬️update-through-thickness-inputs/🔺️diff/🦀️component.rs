//! 🔺️ `update-through-thickness-inputs` — sparse diff construction.

use super::mutation::UpdateThroughThicknessInputs;
use crate::artifacts::en1993::{En1993Diff, En1993Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &UpdateThroughThicknessInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_t10_actual_thickness_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("T10 actual thickness mm must be a finite number, got {}.", payload.new_t10_actual_thickness_mm), Vec::<String>::new());
    }
    if !payload.new_t10_t_ed_c.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("T10 t ed c must be a finite number, got {}.", payload.new_t10_t_ed_c), Vec::<String>::new());
    }
    if base.t10_steel_subgrade == payload.new_t10_steel_subgrade && base.t10_actual_thickness_mm == payload.new_t10_actual_thickness_mm && base.t10_t_ed_c == payload.new_t10_t_ed_c {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "This facet already has these values.");
    }
    protocol::MutationOutcome::new(En1993Diff { t10_steel_subgrade: Some(payload.new_t10_steel_subgrade.clone()), t10_actual_thickness_mm: Some(payload.new_t10_actual_thickness_mm), t10_t_ed_c: Some(payload.new_t10_t_ed_c), ..Default::default() })
}
//#endregion 🔖️Diff
