//! 🔺️ `change-water-well-radius` diff: patches `well_radius` of the addressed layer water row.

use crate::change_water_well_radius::ChangeWaterWellRadius;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997LayerWatersPatch, En1997LayerWatersRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeWaterWellRadius, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_well_radius.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.layer_waters.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("water {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.well_radius == payload.new_well_radius {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The layer water already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { layer_waters: Some(En1997LayerWatersRows::modification(&payload.id, En1997LayerWatersPatch { well_radius: Some(payload.new_well_radius), ..Default::default() })), ..Default::default() })
}
