//! 📍️ `change-pier-base-soil-unit-weight-n-m3` diff: patches `soil_unit_weight_n_m3` of the addressed pier_base row.

use crate::change_pier_base_soil_unit_weight_n_m3::ChangePierBaseSoilUnitWeightNM3;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998PierBaseDelta, En1998PierBasePatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangePierBaseSoilUnitWeightNM3, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_soil_unit_weight_n_m3.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.pier_bases.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("pier_base {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.soil_unit_weight_n_m3 == payload.new_soil_unit_weight_n_m3 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The pier_base already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { pier_bases: En1998PierBaseDelta::modification(&payload.id, En1998PierBasePatch { soil_unit_weight_n_m3: Some(payload.new_soil_unit_weight_n_m3), ..Default::default() }), ..Default::default() })
}
