use crate::change_taper_rod_resistance::ChangeTaperRodResistance;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeTaperRodResistance, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_taper_rod_resistance::ChangeTaperRodResistance> {
    Ok(match base.tapers.iter().find(|item| item.id == payload.taper_id) {
        Some(item) => vec![Op::from(ChangeTaperRodResistance { taper_id: payload.taper_id.clone(), new_value: item.rod_resistance_n })],
        None => Vec::new(),
    })
}
