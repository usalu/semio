use crate::change_truss_web_depth::ChangeTrussWebDepth;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeTrussWebDepth, base: &En1995Snapshot) -> Option<crate::change_truss_web_depth::ChangeTrussWebDepth> {
    match base.trusses.iter().find(|item| item.id == payload.truss_id) {
        Some(item) => Some(ChangeTrussWebDepth { truss_id: payload.truss_id.clone(), new_value: item.web_depth_m }),
        None => None,
    }
}
