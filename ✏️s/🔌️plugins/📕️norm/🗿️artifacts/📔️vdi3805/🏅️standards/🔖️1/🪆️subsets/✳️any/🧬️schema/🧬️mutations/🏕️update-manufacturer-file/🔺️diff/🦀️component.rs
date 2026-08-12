//! 🔺️ `update-manufacturer-file` — sparse diff construction.

use super::mutation::UpdateManufacturerFile;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateManufacturerFile, _base: &Vdi3805Snapshot) -> Vdi3805Diff {
    Vdi3805Diff { manufacturer_file: Some(payload.new_manufacturer_file.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
