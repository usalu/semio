//! ↩️ `rename-manufacturer` — undo restores BASE's manufacturer preferred name.

use super::mutation::RenameManufacturer;
use crate::artifacts::iso16757::{Iso16757Mutation, Iso16757Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &RenameManufacturer, base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    vec![Iso16757Mutation::RenameManufacturer(RenameManufacturer { new_name: base.catalogue.manufacturer.names.preferred.text.clone() })]
}
//#endregion 🔖️Inverse
