use crate::change_diaphragm_shear_resistance::ChangeDiaphragmShearResistance;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeDiaphragmShearResistance, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_diaphragm_shear_resistance::ChangeDiaphragmShearResistance> {
    Ok(match base.diaphragms.iter().find(|item| item.id == payload.diaphragm_id) {
        Some(item) => vec![Op::from(ChangeDiaphragmShearResistance { diaphragm_id: payload.diaphragm_id.clone(), new_value: item.shear_resistance_n_per_m })],
        None => Vec::new(),
    })
}
