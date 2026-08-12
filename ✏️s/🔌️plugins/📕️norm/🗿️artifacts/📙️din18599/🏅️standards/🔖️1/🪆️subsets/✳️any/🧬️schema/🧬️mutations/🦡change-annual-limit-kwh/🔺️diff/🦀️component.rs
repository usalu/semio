//! 🔺️ `change-annual-limit-kwh` sparse diff construction — writes only `Din18599Diff.annual_limit_kwh` from the payload.

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::change_annual_limit_kwh::mutation::ChangeAnnualLimitKwh;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnnualLimitKwh, _base: &Din18599Snapshot) -> Din18599Diff {
    Din18599Diff { annual_limit_kwh: Some(payload.new_annual_limit_kwh.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
