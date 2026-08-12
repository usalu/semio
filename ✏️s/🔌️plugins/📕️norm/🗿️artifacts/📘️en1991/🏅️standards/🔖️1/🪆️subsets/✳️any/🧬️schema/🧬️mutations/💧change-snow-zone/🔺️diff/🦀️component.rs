//! 🔺️ `change-snow-zone` — sparse diff construction.

use super::mutation::ChangeSnowZone;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeSnowZone, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { snow_zone: Some(payload.new_snow_zone.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
