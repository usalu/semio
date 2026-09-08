//! ↩️ `change-renewable-kwh` inverse — restores the pre-change `renewable_kwh` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_renewable_kwh::ChangeRenewableKwh;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeRenewableKwh, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::ChangeRenewableKwh(ChangeRenewableKwh { new_renewable_kwh: base.renewable_kwh })]
}
//#endregion 🔖️Inverse
