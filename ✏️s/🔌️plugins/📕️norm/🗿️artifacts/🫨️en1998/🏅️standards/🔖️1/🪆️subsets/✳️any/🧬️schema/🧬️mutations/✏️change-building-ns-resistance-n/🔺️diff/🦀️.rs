//! ✏️ `change-building-ns-resistance-n` diff: patches `nonstructural_resistance_n` of the addressed building row.

use crate::change_building_ns_resistance_n::ChangeBuildingNsResistanceN;
use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeBuildingNsResistanceN, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_nonstructural_resistance_n.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.buildings.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("building {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.nonstructural_resistance_n == payload.new_nonstructural_resistance_n {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The building already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&payload.id, En1998BuildingPatch { nonstructural_resistance_n: Some(payload.new_nonstructural_resistance_n), ..Default::default() }), ..Default::default() })
}
