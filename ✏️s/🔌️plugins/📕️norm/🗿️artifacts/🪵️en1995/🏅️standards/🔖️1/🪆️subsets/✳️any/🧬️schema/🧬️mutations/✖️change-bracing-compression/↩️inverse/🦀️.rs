use crate::change_bracing_compression::ChangeBracingCompression;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeBracingCompression, base: &En1995Snapshot) -> Option<crate::change_bracing_compression::ChangeBracingCompression> {
    match base.bracings.iter().find(|item| item.id == payload.bracing_id) {
        Some(item) => Some(ChangeBracingCompression { bracing_id: payload.bracing_id.clone(), new_value: item.compression_force_n }),
        None => None,
    }
}
