use crate::change_diaphragm_method::ChangeDiaphragmMethod;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDiaphragmMethod, base: &En1995Snapshot) -> Option<crate::change_diaphragm_method::ChangeDiaphragmMethod> {
    match base.diaphragms.iter().find(|item| item.id == payload.diaphragm_id) {
        Some(item) => Some(ChangeDiaphragmMethod { diaphragm_id: payload.diaphragm_id.clone(), new_value: item.method }),
        None => None,
    }
}
