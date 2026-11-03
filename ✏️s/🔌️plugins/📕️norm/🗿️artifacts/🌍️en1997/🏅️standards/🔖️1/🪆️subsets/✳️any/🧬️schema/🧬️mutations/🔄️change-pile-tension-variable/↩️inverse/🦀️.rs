//! ↩️ `change-pile-tension-variable` inverse: restores the previous `tension_variable`.

use crate::change_pile_tension_variable::ChangePileTensionVariable;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileTensionVariable, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_tension_variable::ChangePileTensionVariable> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileTensionVariable { id: payload.id.clone(), new_tension_variable: row.tension_variable })],
        None => Vec::new(),
    })
}
