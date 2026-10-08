//! 🔺️ `rename-catalogue` — sparse diff construction.

use super::mutation::RenameCatalogue;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff};

//#region 🔖️Diff

pub fn diff(payload: &RenameCatalogue, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.metadata.names.preferred.text == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Catalogue already has that name.");
    }
    protocol::MutationOutcome::new(Iso16757Diff { catalogue_name: Some(payload.new_name.clone()), ..Default::default() })
}
