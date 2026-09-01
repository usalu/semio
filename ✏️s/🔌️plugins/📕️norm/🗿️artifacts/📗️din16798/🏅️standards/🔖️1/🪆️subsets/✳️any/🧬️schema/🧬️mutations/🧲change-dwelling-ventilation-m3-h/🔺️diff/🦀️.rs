//! 🔺️ `change-dwelling-ventilation-m3-h` sparse diff construction — writes only `Din16798Diff.dwelling_ventilation_m3_h` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_dwelling_ventilation_m3_h::ChangeDwellingVentilationM3H;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeDwellingVentilationM3H, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_dwelling_ventilation_m3_h.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Dwelling ventilation air flow must be a finite number, got {}.", payload.new_dwelling_ventilation_m3_h), Vec::<String>::new());
    }
    if base.dwelling_ventilation_m3_h == payload.new_dwelling_ventilation_m3_h {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Dwelling ventilation air flow is already {}.", payload.new_dwelling_ventilation_m3_h));
    }
    protocol::MutationOutcome::new(Din16798Diff { dwelling_ventilation_m3_h: Some(payload.new_dwelling_ventilation_m3_h.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
