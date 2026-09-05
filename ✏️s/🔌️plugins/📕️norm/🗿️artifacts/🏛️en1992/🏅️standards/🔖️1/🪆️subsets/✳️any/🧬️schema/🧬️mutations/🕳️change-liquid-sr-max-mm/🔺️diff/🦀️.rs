//! 🔺️ `change-liquid-sr-max-mm` sparse diff construction — writes only `En1992Diff.liquid_s_r_max_mm` from the payload.

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::change_liquid_s_r_max_mm::ChangeLiquidSRMaxMm;
use crate::artifacts::en1992::En1992Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeLiquidSRMaxMm, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_liquid_s_r_max_mm.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Liquid sr max mm must be a finite number.", Vec::<String>::new());
    }
    if base.liquid_s_r_max_mm == payload.new_liquid_s_r_max_mm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Liquid sr max mm already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { liquid_s_r_max_mm: Some(payload.new_liquid_s_r_max_mm.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
