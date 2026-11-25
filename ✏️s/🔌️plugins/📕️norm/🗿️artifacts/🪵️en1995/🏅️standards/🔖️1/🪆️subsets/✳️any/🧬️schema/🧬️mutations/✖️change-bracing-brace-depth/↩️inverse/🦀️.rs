use crate::change_bracing_brace_depth::ChangeBracingBraceDepth;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeBracingBraceDepth, base: &En1995Snapshot) -> Option<crate::change_bracing_brace_depth::ChangeBracingBraceDepth> {
    match base.bracings.iter().find(|item| item.id == payload.bracing_id) {
        Some(item) => Some(ChangeBracingBraceDepth { bracing_id: payload.bracing_id.clone(), new_value: item.brace_depth_m }),
        None => None,
    }
}
