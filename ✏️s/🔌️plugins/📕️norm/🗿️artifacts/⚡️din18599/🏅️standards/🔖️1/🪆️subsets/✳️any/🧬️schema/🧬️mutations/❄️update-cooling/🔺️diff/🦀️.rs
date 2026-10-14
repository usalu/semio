//! 🔺️ `update-cooling` sparse diff.

use crate::update_cooling::UpdateCooling;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_artifact_norm_din18599_diff::{Din18599Diff, Din18599CoolingPatch, Din18599CoolingPatchPlantValue};

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
