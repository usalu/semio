//! ↩️ `change-pile-alpha-s` inverse: restores the previous `alpha_s`.

use crate::change_pile_alpha_s::ChangePileAlphaS;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileAlphaS, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_alpha_s::ChangePileAlphaS> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileAlphaS { id: payload.id.clone(), new_alpha_s: row.alpha_s })],
        None => Vec::new(),
    })
}
