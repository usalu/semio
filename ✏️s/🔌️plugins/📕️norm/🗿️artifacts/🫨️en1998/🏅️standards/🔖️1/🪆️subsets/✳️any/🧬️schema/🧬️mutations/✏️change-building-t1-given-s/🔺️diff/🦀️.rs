//! ✏️ `change-building-t1-given-s` diff: patches `t1_given_s` of the addressed building row.

use crate::change_building_t1_given_s::ChangeBuildingT1GivenS;
use semio_s_artifact_norm_en1998_diff::{En1998BuildingDelta, En1998BuildingPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeBuildingT1GivenS, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_t1_given_s.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.buildings.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("building {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.t1_given_s == payload.new_t1_given_s {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The building already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { buildings: En1998BuildingDelta::modification(&payload.id, En1998BuildingPatch { t1_given_s: Some(payload.new_t1_given_s), ..Default::default() }), ..Default::default() })
}
