//! ↩️ `change-annual-limit-kwh` inverse — restores the pre-change `annual_limit_kwh` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::din18599::mutations::change_annual_limit_kwh::mutation::ChangeAnnualLimitKwh;
use crate::artifacts::din18599::mutations::Din18599Mutation;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeAnnualLimitKwh, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::ChangeAnnualLimitKwh(ChangeAnnualLimitKwh { new_annual_limit_kwh: base.annual_limit_kwh.clone() })]
}
//#endregion 🔖️Inverse
