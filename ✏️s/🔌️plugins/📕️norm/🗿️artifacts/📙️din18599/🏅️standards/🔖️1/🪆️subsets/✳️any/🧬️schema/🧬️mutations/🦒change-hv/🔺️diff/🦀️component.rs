//! 🔺️ `change-hv` sparse diff construction — writes only `Din18599Diff.h_v` from the payload.

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::change_h_v::mutation::ChangeHV;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHV, _base: &Din18599Snapshot) -> Din18599Diff {
    Din18599Diff { h_v: Some(payload.new_h_v.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
