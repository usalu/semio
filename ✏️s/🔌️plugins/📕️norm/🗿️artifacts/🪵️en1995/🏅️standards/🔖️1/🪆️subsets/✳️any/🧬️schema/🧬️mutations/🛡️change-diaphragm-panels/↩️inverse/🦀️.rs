use crate::change_diaphragm_panels::ChangeDiaphragmPanels;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeDiaphragmPanels, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_diaphragm_panels::ChangeDiaphragmPanels> {
    Ok(match base.diaphragms.iter().find(|item| item.id == payload.diaphragm_id) {
        Some(item) => vec![Op::from(ChangeDiaphragmPanels { diaphragm_id: payload.diaphragm_id.clone(), new_value: item.panels })],
        None => Vec::new(),
    })
}
