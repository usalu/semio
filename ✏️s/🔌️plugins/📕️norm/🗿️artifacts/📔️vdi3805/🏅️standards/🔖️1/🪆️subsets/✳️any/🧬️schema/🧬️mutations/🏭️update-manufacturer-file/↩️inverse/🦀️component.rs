//! ↩️ `update-manufacturer-file` — undo restores BASE's whole header facet.

use super::mutation::UpdateManufacturerFile;
use crate::artifacts::vdi3805::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &UpdateManufacturerFile, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    vec![Vdi3805Mutation::UpdateManufacturerFile(UpdateManufacturerFile { new_manufacturer_file: base.manufacturer_file.clone() })]
}
//#endregion 🔖️Inverse
