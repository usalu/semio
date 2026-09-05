//! 🔺️ `rename-catalogue` — sparse diff construction.

use super::mutation::RenameCatalogue;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &RenameCatalogue, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.metadata.names.preferred.text == payload.new_name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Catalogue already has that name.");
    }
    let mut catalogue = base.catalogue.clone();
    catalogue.metadata.names.preferred.text = payload.new_name.clone();
    protocol::MutationOutcome::new(Iso16757Diff { catalogue: Some(catalogue), ..Default::default() })
}
//#endregion 🔖️Diff
