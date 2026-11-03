//! ↩️ `change-pile-diameter` inverse: restores the previous `diameter`.

use crate::change_pile_diameter::ChangePileDiameter;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileDiameter, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_diameter::ChangePileDiameter> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileDiameter { id: payload.id.clone(), new_diameter: row.diameter })],
        None => Vec::new(),
    })
}
