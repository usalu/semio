//! ↩️ `change-renewable-kwh` inverse — restores the pre-change `renewable_kwh` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din18599::mutations::change_renewable_kwh::mutation::ChangeRenewableKwh;
use crate::artifacts::din18599::mutations::Din18599Mutation;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeRenewableKwh, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::ChangeRenewableKwh(ChangeRenewableKwh { new_renewable_kwh: base.renewable_kwh.clone() })]
}
//#endregion 🔖️Inverse
