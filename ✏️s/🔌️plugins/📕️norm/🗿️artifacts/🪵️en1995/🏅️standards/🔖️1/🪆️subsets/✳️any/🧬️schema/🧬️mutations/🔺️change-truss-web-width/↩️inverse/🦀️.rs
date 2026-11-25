use crate::change_truss_web_width::ChangeTrussWebWidth;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeTrussWebWidth, base: &En1995Snapshot) -> Option<crate::change_truss_web_width::ChangeTrussWebWidth> {
    match base.trusses.iter().find(|item| item.id == payload.truss_id) {
        Some(item) => Some(ChangeTrussWebWidth { truss_id: payload.truss_id.clone(), new_value: item.web_width_m }),
        None => None,
    }
}
