//! 🔺️ `rename-catalogue` — sparse diff construction.

use super::mutation::RenameCatalogue;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &RenameCatalogue, base: &Iso16757Snapshot) -> Iso16757Diff {
    let mut catalogue = base.catalogue.clone();
    catalogue.metadata.names.preferred.text = payload.new_name.clone();
    Iso16757Diff { catalogue: Some(catalogue), ..Default::default() }
}
//#endregion 🔖️Diff
