use crate::change_bracing_brace_width::ChangeBracingBraceWidth;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeBracingBraceWidth, base: &En1995Snapshot) -> Option<crate::change_bracing_brace_width::ChangeBracingBraceWidth> {
    match base.bracings.iter().find(|item| item.id == payload.bracing_id) {
        Some(item) => Some(ChangeBracingBraceWidth { bracing_id: payload.bracing_id.clone(), new_value: item.brace_width_m }),
        None => None,
    }
}
