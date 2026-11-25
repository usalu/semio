use crate::change_taper_rod_resistance::ChangeTaperRodResistance;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeTaperRodResistance, base: &En1995Snapshot) -> Option<crate::change_taper_rod_resistance::ChangeTaperRodResistance> {
    match base.tapers.iter().find(|item| item.id == payload.taper_id) {
        Some(item) => Some(ChangeTaperRodResistance { taper_id: payload.taper_id.clone(), new_value: item.rod_resistance_n }),
        None => None,
    }
}
