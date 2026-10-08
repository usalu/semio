//! 🔺️ `specify-heating-system` sparse diff.

use crate::mutations::specify_heating_system::SpecifyHeatingSystem;
use crate::Din18599Snapshot;
use crate::diff::{Din18599Diff, Din18599HeatingPatch};

pub fn diff(payload: &SpecifyHeatingSystem, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {

    if base.heating == payload.new_heating {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "heating already has this value.");
    }
    let (old, new) = (&base.heating, &payload.new_heating);
    protocol::MutationOutcome::new(Din18599Diff {
        heating: Some(Din18599HeatingPatch {
            generation_efficiency: (old.generation_efficiency != new.generation_efficiency).then(|| new.generation_efficiency.clone()),
            distribution_efficiency: (old.distribution_efficiency != new.distribution_efficiency).then(|| new.distribution_efficiency.clone()),
            storage_efficiency: (old.storage_efficiency != new.storage_efficiency).then(|| new.storage_efficiency.clone()),
            transfer_efficiency: (old.transfer_efficiency != new.transfer_efficiency).then(|| new.transfer_efficiency.clone()),
            energy_carrier: (old.energy_carrier != new.energy_carrier).then(|| new.energy_carrier.clone()),
        }),
        ..Default::default()
    })
}
