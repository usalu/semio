use crate::change_diaphragm_fastener_capacity::ChangeDiaphragmFastenerCapacity;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDiaphragmFastenerCapacity, base: &En1995Snapshot) -> Option<crate::change_diaphragm_fastener_capacity::ChangeDiaphragmFastenerCapacity> {
    match base.diaphragms.iter().find(|item| item.id == payload.diaphragm_id) {
        Some(item) => Some(ChangeDiaphragmFastenerCapacity { diaphragm_id: payload.diaphragm_id.clone(), new_value: item.fastener_capacity_n }),
        None => None,
    }
}
