use crate::change_diaphragm_force::ChangeDiaphragmForce;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeDiaphragmForce, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_diaphragm_force::ChangeDiaphragmForce> {
    Ok(match base.diaphragms.iter().find(|item| item.id == payload.diaphragm_id) {
        Some(item) => vec![Op::from(ChangeDiaphragmForce { diaphragm_id: payload.diaphragm_id.clone(), new_value: item.diaphragm_force_n })],
        None => Vec::new(),
    })
}
