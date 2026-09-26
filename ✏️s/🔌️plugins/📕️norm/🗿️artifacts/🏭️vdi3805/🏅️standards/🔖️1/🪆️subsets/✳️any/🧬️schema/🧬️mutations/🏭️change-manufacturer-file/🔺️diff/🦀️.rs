//! 🔺️ `change-manufacturer-file` — sparse diff construction.

use super::ChangeManufacturerFile;
use crate::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
/// 🏷️ Sparse header patch: `manufacturer_file` applies onto `catalog.file` (single stored header).
pub fn diff(payload: &ChangeManufacturerFile, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    if base.catalog.file == payload.new_manufacturer_file {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Manufacturer file already has this value.");
    }
    protocol::MutationOutcome::new(Vdi3805Diff { manufacturer_file: Some(payload.new_manufacturer_file.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
