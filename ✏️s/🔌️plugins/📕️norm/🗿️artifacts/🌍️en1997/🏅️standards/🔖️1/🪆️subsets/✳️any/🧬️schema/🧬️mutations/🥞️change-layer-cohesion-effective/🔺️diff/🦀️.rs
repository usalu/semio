//! 🔺️ `change-layer-cohesion-effective` diff: patches `cohesion_effective` of the addressed soil layer row.

use crate::change_layer_cohesion_effective::ChangeLayerCohesionEffective;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997LayersPatch, En1997LayersRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeLayerCohesionEffective, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_cohesion_effective.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.layers.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("layer {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.cohesion_effective == payload.new_cohesion_effective {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The soil layer already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { layers: Some(En1997LayersRows::modification(&payload.id, En1997LayersPatch { cohesion_effective: Some(payload.new_cohesion_effective), ..Default::default() })), ..Default::default() })
}
