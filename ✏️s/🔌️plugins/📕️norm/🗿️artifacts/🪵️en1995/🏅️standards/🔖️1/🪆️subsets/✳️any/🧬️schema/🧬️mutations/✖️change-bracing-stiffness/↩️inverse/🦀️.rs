use crate::change_bracing_stiffness::ChangeBracingStiffness;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeBracingStiffness, base: &En1995Snapshot) -> Option<crate::change_bracing_stiffness::ChangeBracingStiffness> {
    match base.bracings.iter().find(|item| item.id == payload.bracing_id) {
        Some(item) => Some(ChangeBracingStiffness { bracing_id: payload.bracing_id.clone(), new_value: item.stiffness_n_per_m }),
        None => None,
    }
}
