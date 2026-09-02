//! 🔺️ `rename-manufacturer` — sparse diff construction.

use super::mutation::RenameManufacturer;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &RenameManufacturer, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.manufacturer.names.preferred.text == payload.new_name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Manufacturer already has that name.");
    }
    let mut catalogue = base.catalogue.clone();
    catalogue.manufacturer.names.preferred.text = payload.new_name.clone();
    protocol::MutationOutcome::new(Iso16757Diff { catalogue: Some(catalogue), ..Default::default() })
}
//#endregion 🔖️Diff
