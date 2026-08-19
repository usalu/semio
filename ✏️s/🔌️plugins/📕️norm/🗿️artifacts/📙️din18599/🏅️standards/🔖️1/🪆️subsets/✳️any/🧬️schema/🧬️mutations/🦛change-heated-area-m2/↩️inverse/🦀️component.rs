//! ↩️ `change-heated-area-m2` inverse — restores the pre-change `heated_area_m2` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din18599::mutations::change_heated_area_m2::mutation::ChangeHeatedAreaM2;
use crate::artifacts::din18599::mutations::Din18599Mutation;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeHeatedAreaM2, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::ChangeHeatedAreaM2(ChangeHeatedAreaM2 { new_heated_area_m2: base.heated_area_m2.clone() })]
}
//#endregion 🔖️Inverse
