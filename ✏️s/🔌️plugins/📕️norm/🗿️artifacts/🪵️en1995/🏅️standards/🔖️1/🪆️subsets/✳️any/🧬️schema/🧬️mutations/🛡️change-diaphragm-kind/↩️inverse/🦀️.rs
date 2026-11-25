use crate::change_diaphragm_kind::ChangeDiaphragmKind;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDiaphragmKind, base: &En1995Snapshot) -> Option<crate::change_diaphragm_kind::ChangeDiaphragmKind> {
    match base.diaphragms.iter().find(|item| item.id == payload.diaphragm_id) {
        Some(item) => Some(ChangeDiaphragmKind { diaphragm_id: payload.diaphragm_id.clone(), new_value: item.kind }),
        None => None,
    }
}
