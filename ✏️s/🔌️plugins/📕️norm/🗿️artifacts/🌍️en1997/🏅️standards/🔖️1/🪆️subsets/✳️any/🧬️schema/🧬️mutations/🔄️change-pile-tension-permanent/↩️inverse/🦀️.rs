//! ↩️ `change-pile-tension-permanent` inverse: restores the previous `tension_permanent`.

use crate::change_pile_tension_permanent::ChangePileTensionPermanent;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileTensionPermanent, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_tension_permanent::ChangePileTensionPermanent> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileTensionPermanent { id: payload.id.clone(), new_tension_permanent: row.tension_permanent })],
        None => Vec::new(),
    })
}
