//! ↩️ `change-system-losses-kwh` inverse — restores the pre-change `system_losses_kwh` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din18599::mutations::change_system_losses_kwh::mutation::ChangeSystemLossesKwh;
use crate::artifacts::din18599::mutations::Din18599Mutation;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSystemLossesKwh, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::ChangeSystemLossesKwh(ChangeSystemLossesKwh { new_system_losses_kwh: base.system_losses_kwh.clone() })]
}
//#endregion 🔖️Inverse
