use crate::change_bracing_stabilised_count::ChangeBracingStabilisedCount;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeBracingStabilisedCount, base: &En1995Snapshot) -> Option<crate::change_bracing_stabilised_count::ChangeBracingStabilisedCount> {
    match base.bracings.iter().find(|item| item.id == payload.bracing_id) {
        Some(item) => Some(ChangeBracingStabilisedCount { bracing_id: payload.bracing_id.clone(), new_value: item.stabilised_count }),
        None => None,
    }
}
