use crate::change_taper_lamella::ChangeTaperLamella;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeTaperLamella, base: &En1995Snapshot) -> Option<crate::change_taper_lamella::ChangeTaperLamella> {
    match base.tapers.iter().find(|item| item.id == payload.taper_id) {
        Some(item) => Some(ChangeTaperLamella { taper_id: payload.taper_id.clone(), new_value: item.lamella_thickness_m }),
        None => None,
    }
}
