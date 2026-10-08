//! 🔺️ `specify-dhw-system` sparse diff.

use crate::mutations::specify_dhw_system::SpecifyDhwSystem;
use crate::Din18599Snapshot;
use crate::diff::{Din18599Diff, Din18599DhwPatch};

pub fn diff(payload: &SpecifyDhwSystem, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {

    if base.dhw == payload.new_dhw {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "dhw already has this value.");
    }
    let (old, new) = (&base.dhw, &payload.new_dhw);
    protocol::MutationOutcome::new(Din18599Diff {
        dhw: Some(Din18599DhwPatch {
            specific_demand_kwh_person_a: (old.specific_demand_kwh_person_a != new.specific_demand_kwh_person_a).then(|| new.specific_demand_kwh_person_a.clone()),
            storage_loss_kwh_a: (old.storage_loss_kwh_a != new.storage_loss_kwh_a).then(|| new.storage_loss_kwh_a.clone()),
            distribution_loss_kwh_a: (old.distribution_loss_kwh_a != new.distribution_loss_kwh_a).then(|| new.distribution_loss_kwh_a.clone()),
            energy_carrier: (old.energy_carrier != new.energy_carrier).then(|| new.energy_carrier.clone()),
        }),
        ..Default::default()
    })
}
