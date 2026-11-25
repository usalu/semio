use crate::change_bracing_kind::ChangeBracingKind;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeBracingKind, base: &En1995Snapshot) -> Option<crate::change_bracing_kind::ChangeBracingKind> {
    match base.bracings.iter().find(|item| item.id == payload.bracing_id) {
        Some(item) => Some(ChangeBracingKind { bracing_id: payload.bracing_id.clone(), new_value: item.kind }),
        None => None,
    }
}
