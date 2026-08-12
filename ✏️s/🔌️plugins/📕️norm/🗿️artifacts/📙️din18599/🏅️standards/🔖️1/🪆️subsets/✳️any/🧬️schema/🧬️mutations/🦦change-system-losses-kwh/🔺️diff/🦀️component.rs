//! 🔺️ `change-system-losses-kwh` sparse diff construction — writes only `Din18599Diff.system_losses_kwh` from the payload.

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::change_system_losses_kwh::mutation::ChangeSystemLossesKwh;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSystemLossesKwh, _base: &Din18599Snapshot) -> Din18599Diff {
    Din18599Diff { system_losses_kwh: Some(payload.new_system_losses_kwh.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
