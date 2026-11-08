//! 🔺️ `change-fire-insulation-fire-id` diff: patches `fire_id` of the addressed fire_insulation stiffener row.

use crate::change_fire_insulation_fire_id::ChangeFireInsulationFireId;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999FireInsulationsPatch, En1999FireInsulationsRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeFireInsulationFireId, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(r0) = base.fire_insulations.iter().find(|row| row.id == payload.fire_insulation_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fire-insulation", Vec::<String>::new());
    };
    if r0.fire_id == payload.new_fire_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fire_insulation stiffener already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { fire_insulations: Some(En1999FireInsulationsRows::modification(&payload.fire_insulation_id, En1999FireInsulationsPatch { fire_id: Some(payload.new_fire_id.clone()), ..Default::default() })), ..Default::default() })
}
