//! 🔺️ `change-cellar-ventilation-m3-h` sparse diff construction — writes only `Din16798Diff.cellar_ventilation_m3_h` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_cellar_ventilation_m3_h::ChangeCellarVentilationM3H;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeCellarVentilationM3H, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_cellar_ventilation_m3_h.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Cellar ventilation air flow must be a finite number, got {}.", payload.new_cellar_ventilation_m3_h), Vec::<String>::new());
    }
    if base.cellar_ventilation_m3_h == payload.new_cellar_ventilation_m3_h {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Cellar ventilation air flow is already {}.", payload.new_cellar_ventilation_m3_h));
    }
    protocol::MutationOutcome::new(Din16798Diff { cellar_ventilation_m3_h: Some(payload.new_cellar_ventilation_m3_h.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
