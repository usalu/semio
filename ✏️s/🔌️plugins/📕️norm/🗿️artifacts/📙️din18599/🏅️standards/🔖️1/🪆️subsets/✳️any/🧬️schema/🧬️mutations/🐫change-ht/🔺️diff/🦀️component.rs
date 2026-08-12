//! 🔺️ `change-ht` sparse diff construction — writes only `Din18599Diff.h_t` from the payload.

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::change_h_t::mutation::ChangeHT;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHT, _base: &Din18599Snapshot) -> Din18599Diff {
    Din18599Diff { h_t: Some(payload.new_h_t.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
