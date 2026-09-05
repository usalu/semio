//! 🔺️ `change-duct-leakage-m3-sm2` sparse diff construction — writes only `Din16798Diff.duct_leakage_m3_s_m2` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_duct_leakage_m3_s_m2::ChangeDuctLeakageM3SM2;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeDuctLeakageM3SM2, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_duct_leakage_m3_s_m2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Duct leakage rate must be a finite number, got {}.", payload.new_duct_leakage_m3_s_m2), Vec::<String>::new());
    }
    if base.duct_leakage_m3_s_m2 == payload.new_duct_leakage_m3_s_m2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Duct leakage rate is already {}.", payload.new_duct_leakage_m3_s_m2));
    }
    protocol::MutationOutcome::new(Din16798Diff { duct_leakage_m3_s_m2: Some(payload.new_duct_leakage_m3_s_m2.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
