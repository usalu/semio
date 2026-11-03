//! ✏️ `change-building-accidental-ecc-ratio` diff: patches `accidental_eccentricity_ratio` of the addressed building row.

use crate::change_building_accidental_ecc_ratio::ChangeBuildingAccidentalEccRatio;
use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeBuildingAccidentalEccRatio, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_accidental_eccentricity_ratio.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.buildings.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("building {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.accidental_eccentricity_ratio == payload.new_accidental_eccentricity_ratio {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The building already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&payload.id, En1998BuildingPatch { accidental_eccentricity_ratio: Some(payload.new_accidental_eccentricity_ratio), ..Default::default() }), ..Default::default() })
}
