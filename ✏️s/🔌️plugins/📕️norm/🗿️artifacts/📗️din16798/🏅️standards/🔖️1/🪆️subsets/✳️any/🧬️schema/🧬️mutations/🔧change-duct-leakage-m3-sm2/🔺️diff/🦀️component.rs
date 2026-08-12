//! 🔺️ `change-duct-leakage-m3-sm2` sparse diff construction — writes only `Din16798Diff.duct_leakage_m3_s_m2` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_duct_leakage_m3_s_m2::mutation::ChangeDuctLeakageM3SM2;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeDuctLeakageM3SM2, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { duct_leakage_m3_s_m2: Some(payload.new_duct_leakage_m3_s_m2.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
