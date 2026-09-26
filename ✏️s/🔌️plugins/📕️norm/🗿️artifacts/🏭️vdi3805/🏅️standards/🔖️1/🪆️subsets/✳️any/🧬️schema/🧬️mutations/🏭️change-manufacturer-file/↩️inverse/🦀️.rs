//! ↩️ `change-manufacturer-file` — undo restores BASE's whole header facet.

use super::ChangeManufacturerFile;
use crate::{Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeManufacturerFile, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    vec![Vdi3805Mutation::ChangeManufacturerFile(ChangeManufacturerFile { new_manufacturer_file: base.catalog.file.clone() })]
}
//#endregion 🔖️Inverse
