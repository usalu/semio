use crate::change_truss_bracing_capacity::ChangeTrussBracingCapacity;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeTrussBracingCapacity, base: &En1995Snapshot) -> Option<crate::change_truss_bracing_capacity::ChangeTrussBracingCapacity> {
    match base.trusses.iter().find(|item| item.id == payload.truss_id) {
        Some(item) => Some(ChangeTrussBracingCapacity { truss_id: payload.truss_id.clone(), new_value: item.bracing_capacity_n_per_m }),
        None => None,
    }
}
