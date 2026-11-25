use crate::change_diaphragm_force::ChangeDiaphragmForce;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDiaphragmForce, base: &En1995Snapshot) -> Option<crate::change_diaphragm_force::ChangeDiaphragmForce> {
    match base.diaphragms.iter().find(|item| item.id == payload.diaphragm_id) {
        Some(item) => Some(ChangeDiaphragmForce { diaphragm_id: payload.diaphragm_id.clone(), new_value: item.diaphragm_force_n }),
        None => None,
    }
}
