//! 🔺️ `update-climate` sparse diff construction — writes only `Din18599Diff.climate` from the payload.

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::update_climate::mutation::UpdateClimate;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &UpdateClimate, _base: &Din18599Snapshot) -> Din18599Diff {
    Din18599Diff { climate: Some(payload.new_climate.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
