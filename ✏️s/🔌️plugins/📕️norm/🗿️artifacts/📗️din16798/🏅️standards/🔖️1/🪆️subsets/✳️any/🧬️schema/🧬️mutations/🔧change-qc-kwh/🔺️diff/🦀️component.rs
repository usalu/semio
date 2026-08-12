//! 🔺️ `change-qc-kwh` sparse diff construction — writes only `Din16798Diff.q_c_kwh` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_q_c_kwh::mutation::ChangeQCKwh;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeQCKwh, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { q_c_kwh: Some(payload.new_q_c_kwh.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
