use crate::change_deck_shear_stress::ChangeDeckShearStress;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeDeckShearStress, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_deck_shear_stress::ChangeDeckShearStress> {
    Ok(match base.decks.iter().find(|item| item.id == payload.deck_id) {
        Some(item) => vec![Op::from(ChangeDeckShearStress { deck_id: payload.deck_id.clone(), new_value: item.shear_stress_pa })],
        None => Vec::new(),
    })
}
