//! ↩️ `change-pile-concrete-class` inverse: restores the previous `concrete_class`.

use crate::change_pile_concrete_class::ChangePileConcreteClass;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileConcreteClass, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_concrete_class::ChangePileConcreteClass> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileConcreteClass { id: payload.id.clone(), new_concrete_class: row.concrete_class })],
        None => Vec::new(),
    })
}
