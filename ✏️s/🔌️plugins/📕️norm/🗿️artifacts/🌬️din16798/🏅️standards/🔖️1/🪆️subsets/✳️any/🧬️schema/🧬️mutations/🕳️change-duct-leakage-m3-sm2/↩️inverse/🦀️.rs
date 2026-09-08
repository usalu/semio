//! ↩️ `change-duct-leakage-m3-sm2` inverse — restores the pre-change `duct_leakage_m3_s_m2` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_duct_leakage_m3_s_m2::ChangeDuctLeakageM3SM2;
use crate::mutations::Din16798Mutation;
use crate::Din16798Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeDuctLeakageM3SM2, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::ChangeDuctLeakageM3SM2(ChangeDuctLeakageM3SM2 { new_duct_leakage_m3_s_m2: base.duct_leakage_m3_s_m2 })]
}
//#endregion 🔖️Inverse
