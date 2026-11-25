use crate::change_diaphragm_length::ChangeDiaphragmLength;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDiaphragmLength, base: &En1995Snapshot) -> Option<crate::change_diaphragm_length::ChangeDiaphragmLength> {
    match base.diaphragms.iter().find(|item| item.id == payload.diaphragm_id) {
        Some(item) => Some(ChangeDiaphragmLength { diaphragm_id: payload.diaphragm_id.clone(), new_value: item.diaphragm_length_m }),
        None => None,
    }
}
