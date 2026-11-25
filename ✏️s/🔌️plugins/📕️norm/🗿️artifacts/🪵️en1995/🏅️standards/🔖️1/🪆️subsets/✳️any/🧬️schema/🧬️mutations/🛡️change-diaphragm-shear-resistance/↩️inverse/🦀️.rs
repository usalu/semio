use crate::change_diaphragm_shear_resistance::ChangeDiaphragmShearResistance;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeDiaphragmShearResistance, base: &En1995Snapshot) -> Option<crate::change_diaphragm_shear_resistance::ChangeDiaphragmShearResistance> {
    match base.diaphragms.iter().find(|item| item.id == payload.diaphragm_id) {
        Some(item) => Some(ChangeDiaphragmShearResistance { diaphragm_id: payload.diaphragm_id.clone(), new_value: item.shear_resistance_n_per_m }),
        None => None,
    }
}
