//! 🔺️ `update-cooling` sparse diff.

use crate::mutations::update_cooling::UpdateCooling;
use crate::Din18599Snapshot;
use crate::diff::{Din18599Diff, Din18599CoolingPatch, Din18599CoolingPatchPlantValue};

pub fn diff(payload: &UpdateCooling, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {

    if base.cooling == payload.new_cooling {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "cooling already has this value.");
    }
    let (old, new) = (&base.cooling, &payload.new_cooling);
    protocol::MutationOutcome::new(Din18599Diff {
        cooling: Some(Din18599CoolingPatch {
            plant: (old.plant != new.plant).then(|| Din18599CoolingPatchPlantValue { value: new.plant.clone() }),
        }),
        ..Default::default()
    })
}
