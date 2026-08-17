//! 🔺️ `update-manufacturer-file` — sparse diff construction.

use super::mutation::UpdateManufacturerFile;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateManufacturerFile, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    if base.manufacturer_file == payload.new_manufacturer_file {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Manufacturer file already has this value.");
    }
    protocol::MutationOutcome::new(Vdi3805Diff { manufacturer_file: Some(payload.new_manufacturer_file.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
