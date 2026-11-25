use crate::change_diaphragm_hold_down::ChangeDiaphragmHoldDown;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDiaphragmHoldDown, base: &En1995Snapshot) -> Option<crate::change_diaphragm_hold_down::ChangeDiaphragmHoldDown> {
    match base.diaphragms.iter().find(|item| item.id == payload.diaphragm_id) {
        Some(item) => Some(ChangeDiaphragmHoldDown { diaphragm_id: payload.diaphragm_id.clone(), new_value: item.hold_down_n }),
        None => None,
    }
}
