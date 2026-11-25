use crate::change_bracing_length::ChangeBracingLength;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeBracingLength, base: &En1995Snapshot) -> Option<crate::change_bracing_length::ChangeBracingLength> {
    match base.bracings.iter().find(|item| item.id == payload.bracing_id) {
        Some(item) => Some(ChangeBracingLength { bracing_id: payload.bracing_id.clone(), new_value: item.length_m }),
        None => None,
    }
}
