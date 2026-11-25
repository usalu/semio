use crate::change_taper_side::ChangeTaperSide;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeTaperSide, base: &En1995Snapshot) -> Option<crate::change_taper_side::ChangeTaperSide> {
    match base.tapers.iter().find(|item| item.id == payload.taper_id) {
        Some(item) => Some(ChangeTaperSide { taper_id: payload.taper_id.clone(), new_value: item.tension_side }),
        None => None,
    }
}
