use crate::change_diaphragm_shear_ed::ChangeDiaphragmShearEd;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDiaphragmShearEd, base: &En1995Snapshot) -> Option<crate::change_diaphragm_shear_ed::ChangeDiaphragmShearEd> {
    match base.diaphragms.iter().find(|item| item.id == payload.diaphragm_id) {
        Some(item) => Some(ChangeDiaphragmShearEd { diaphragm_id: payload.diaphragm_id.clone(), new_value: item.shear_ed_n }),
        None => None,
    }
}
