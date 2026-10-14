//! 🔺️ `rename-manufacturer` — sparse diff construction.

use crate::rename_manufacturer::mutation::RenameManufacturer;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;
use semio_s_artifact_norm_iso16757_diff::Iso16757Diff;

//#region 🔖️Diff

pub fn diff(payload: &RenameManufacturer, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.manufacturer.names.preferred.text == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Manufacturer already has that name.");
    }
    protocol::MutationOutcome::new(Iso16757Diff { manufacturer_name: Some(payload.new_name.clone()), ..Default::default() })
}
