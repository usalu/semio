//! 🔺️ `change-wind-zone` — sparse diff construction.

use super::mutation::ChangeWindZone;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeWindZone, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { wind_zone: Some(payload.new_wind_zone.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
