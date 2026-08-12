//! 🔺️ `change-renewable-kwh` sparse diff construction — writes only `Din18599Diff.renewable_kwh` from the payload.

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::change_renewable_kwh::mutation::ChangeRenewableKwh;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeRenewableKwh, _base: &Din18599Snapshot) -> Din18599Diff {
    Din18599Diff { renewable_kwh: Some(payload.new_renewable_kwh.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
