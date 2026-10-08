//! 🔺️ `rename-manufacturer` — sparse diff construction.

use super::mutation::RenameManufacturer;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff};

//#region 🔖️Diff

pub fn diff(payload: &RenameManufacturer, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.manufacturer.names.preferred.text == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Manufacturer already has that name.");
    }
    protocol::MutationOutcome::new(Iso16757Diff { manufacturer_name: Some(payload.new_name.clone()), ..Default::default() })
}
