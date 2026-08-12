//! 🔺️ `change-climate` — sparse diff construction.

use super::mutation::ChangeClimate;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeClimate, _base: &Din4108Snapshot) -> Din4108Diff {
    Din4108Diff { climate: Some(payload.new_climate), ..Default::default() }
}
//#endregion 🔖️Diff
