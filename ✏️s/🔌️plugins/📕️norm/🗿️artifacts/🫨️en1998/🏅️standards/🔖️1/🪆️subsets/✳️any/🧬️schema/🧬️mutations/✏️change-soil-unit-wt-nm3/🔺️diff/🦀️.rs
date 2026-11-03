//! ✏️ `change-soil-unit-wt-nm3` diff: patches `unit_weight_n_m3` of the addressed soil layer row.

use crate::change_soil_unit_wt_n_m3::ChangeSoilUnitWtNM3;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998SoilLayerDelta, En1998SoilLayerPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeSoilUnitWtNM3, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_unit_weight_n_m3.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.soil_layers.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("soil layer {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.unit_weight_n_m3 == payload.new_unit_weight_n_m3 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The soil layer already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { soil_layers: En1998SoilLayerDelta::modification(&payload.id, En1998SoilLayerPatch { unit_weight_n_m3: Some(payload.new_unit_weight_n_m3), ..Default::default() }), ..Default::default() })
}
