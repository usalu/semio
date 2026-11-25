use crate::change_taper_apex_depth::ChangeTaperApexDepth;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeTaperApexDepth, base: &En1995Snapshot) -> Option<crate::change_taper_apex_depth::ChangeTaperApexDepth> {
    match base.tapers.iter().find(|item| item.id == payload.taper_id) {
        Some(item) => Some(ChangeTaperApexDepth { taper_id: payload.taper_id.clone(), new_value: item.apex_depth_m }),
        None => None,
    }
}
