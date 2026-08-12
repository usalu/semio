//! 🔺️ `change-rh-int` — sparse diff construction.

use super::mutation::ChangeRhInt;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeRhInt, _base: &Din4108Snapshot) -> Din4108Diff {
    Din4108Diff { rh_int: Some(payload.new_rh_int), ..Default::default() }
}
//#endregion 🔖️Diff
