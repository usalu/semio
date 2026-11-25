use crate::change_cutout_rod_resistance::ChangeCutoutRodResistance;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeCutoutRodResistance, base: &En1995Snapshot) -> Option<crate::change_cutout_rod_resistance::ChangeCutoutRodResistance> {
    match base.cutouts.iter().find(|item| item.id == payload.cutout_id) {
        Some(item) => Some(ChangeCutoutRodResistance { cutout_id: payload.cutout_id.clone(), new_value: item.rod_resistance_n }),
        None => None,
    }
}
