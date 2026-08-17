//! 🔺️ `change-energy-carrier` sparse diff construction — writes only `Din18599Diff.energy_carrier` from the payload.

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::change_energy_carrier::mutation::ChangeEnergyCarrier;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeEnergyCarrier, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if base.energy_carrier == payload.new_energy_carrier {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Energy carrier already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { energy_carrier: Some(payload.new_energy_carrier.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
