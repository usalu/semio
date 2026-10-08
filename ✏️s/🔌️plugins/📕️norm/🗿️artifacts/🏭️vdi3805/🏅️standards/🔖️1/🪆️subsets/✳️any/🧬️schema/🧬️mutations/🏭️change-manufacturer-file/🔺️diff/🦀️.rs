//! 🔺️ `change-manufacturer-file` — sparse diff construction.

use super::ChangeManufacturerFile;
use crate::{Vdi3805Snapshot};
use crate::diff::{Vdi3805Diff, Vdi3805ManufacturerFilePatch};

//#region 🔖️Diff
/// 🏷️ Sparse header patch: `manufacturer_file` applies onto `catalog.file` (single stored header).

pub fn diff(payload: &ChangeManufacturerFile, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<Vdi3805Diff> {
    if base.catalog.file == payload.new_manufacturer_file {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Manufacturer file already has this value.");
    }
    let (old, new) = (&base.catalog.file, &payload.new_manufacturer_file);
    protocol::MutationOutcome::new(Vdi3805Diff {
        manufacturer_file: Some(Vdi3805ManufacturerFilePatch {
            header_version: (old.header_version != new.header_version).then(|| new.header_version.clone()),
            manufacturer: (old.manufacturer != new.manufacturer).then(|| new.manufacturer.clone()),
            building_system_number: (old.building_system_number != new.building_system_number).then(|| new.building_system_number.clone()),
            created: (old.created != new.created).then(|| new.created.clone()),
            charset: (old.charset != new.charset).then(|| new.charset.clone()),
            record_count: (old.record_count != new.record_count).then(|| new.record_count.clone()),
            extensions: (old.extensions != new.extensions).then(|| new.extensions.clone()),
        }),
        ..Default::default()
    })
}
