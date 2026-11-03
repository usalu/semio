use crate::change_truss_bracing_capacity::ChangeTrussBracingCapacity;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeTrussBracingCapacity, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_truss_bracing_capacity::ChangeTrussBracingCapacity> {
    Ok(match base.trusses.iter().find(|item| item.id == payload.truss_id) {
        Some(item) => vec![Op::from(ChangeTrussBracingCapacity { truss_id: payload.truss_id.clone(), new_value: item.bracing_capacity_n_per_m })],
        None => Vec::new(),
    })
}
