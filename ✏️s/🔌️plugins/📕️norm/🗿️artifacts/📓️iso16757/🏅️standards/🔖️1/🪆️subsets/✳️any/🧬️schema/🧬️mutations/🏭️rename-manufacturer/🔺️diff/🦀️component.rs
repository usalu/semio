//! 🔺️ `rename-manufacturer` — sparse diff construction.

use super::mutation::RenameManufacturer;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &RenameManufacturer, base: &Iso16757Snapshot) -> Iso16757Diff {
    let mut catalogue = base.catalogue.clone();
    catalogue.manufacturer.names.preferred.text = payload.new_name.clone();
    Iso16757Diff { catalogue: Some(catalogue), ..Default::default() }
}
//#endregion 🔖️Diff
