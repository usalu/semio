use crate::change_diaphragm_shear_ed::ChangeDiaphragmShearEd;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeDiaphragmShearEd, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_diaphragm_shear_ed::ChangeDiaphragmShearEd> {
    Ok(match base.diaphragms.iter().find(|item| item.id == payload.diaphragm_id) {
        Some(item) => vec![Op::from(ChangeDiaphragmShearEd { diaphragm_id: payload.diaphragm_id.clone(), new_value: item.shear_ed_n })],
        None => Vec::new(),
    })
}
