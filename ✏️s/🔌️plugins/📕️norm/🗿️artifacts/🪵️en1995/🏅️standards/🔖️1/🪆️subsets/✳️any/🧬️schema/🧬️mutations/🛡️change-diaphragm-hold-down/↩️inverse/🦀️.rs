use crate::change_diaphragm_hold_down::ChangeDiaphragmHoldDown;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeDiaphragmHoldDown, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_diaphragm_hold_down::ChangeDiaphragmHoldDown> {
    Ok(match base.diaphragms.iter().find(|item| item.id == payload.diaphragm_id) {
        Some(item) => vec![Op::from(ChangeDiaphragmHoldDown { diaphragm_id: payload.diaphragm_id.clone(), new_value: item.hold_down_n })],
        None => Vec::new(),
    })
}
