//! 🔺️ `change-heated-area-m2` sparse diff construction — writes only `Din18599Diff.heated_area_m2` from the payload.

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::change_heated_area_m2::mutation::ChangeHeatedAreaM2;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeHeatedAreaM2, _base: &Din18599Snapshot) -> Din18599Diff {
    Din18599Diff { heated_area_m2: Some(payload.new_heated_area_m2.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
