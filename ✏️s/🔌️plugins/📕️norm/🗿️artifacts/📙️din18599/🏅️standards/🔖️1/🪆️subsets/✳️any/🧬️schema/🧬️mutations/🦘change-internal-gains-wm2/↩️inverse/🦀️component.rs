//! ↩️ `change-internal-gains-wm2` inverse — restores the pre-change `internal_gains_w_m2` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din18599::mutations::change_internal_gains_w_m2::mutation::ChangeInternalGainsWM2;
use crate::artifacts::din18599::mutations::Din18599Mutation;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeInternalGainsWM2, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::ChangeInternalGainsWM2(ChangeInternalGainsWM2 { new_internal_gains_w_m2: base.internal_gains_w_m2.clone() })]
}
//#endregion 🔖️Inverse
