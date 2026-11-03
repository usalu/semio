//! 🔺️ `change-water-permeability` diff: patches `permeability` of the addressed layer water row.

use crate::change_water_permeability::ChangeWaterPermeability;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997LayerWatersPatch, En1997LayerWatersRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeWaterPermeability, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_permeability.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.layer_waters.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("water {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.permeability == payload.new_permeability {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The layer water already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { layer_waters: Some(En1997LayerWatersRows::modification(&payload.id, En1997LayerWatersPatch { permeability: Some(payload.new_permeability), ..Default::default() })), ..Default::default() })
}
