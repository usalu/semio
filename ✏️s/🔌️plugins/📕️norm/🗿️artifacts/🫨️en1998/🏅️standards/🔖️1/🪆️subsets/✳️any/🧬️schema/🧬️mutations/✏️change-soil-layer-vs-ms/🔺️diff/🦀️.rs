//! ✏️ `change-soil-layer-vs-ms` diff: patches `shear_wave_velocity_m_s` of the addressed soil layer row.

use crate::change_soil_layer_vs_m_s::ChangeSoilLayerVsMS;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998SoilLayerDelta, En1998SoilLayerPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeSoilLayerVsMS, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_shear_wave_velocity_m_s.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.soil_layers.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("soil layer {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.shear_wave_velocity_m_s == payload.new_shear_wave_velocity_m_s {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The soil layer already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { soil_layers: En1998SoilLayerDelta::modification(&payload.id, En1998SoilLayerPatch { shear_wave_velocity_m_s: Some(payload.new_shear_wave_velocity_m_s), ..Default::default() }), ..Default::default() })
}
