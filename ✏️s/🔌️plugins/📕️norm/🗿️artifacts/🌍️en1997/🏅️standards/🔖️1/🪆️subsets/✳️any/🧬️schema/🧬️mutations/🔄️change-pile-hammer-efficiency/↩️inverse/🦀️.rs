//! ↩️ `change-pile-hammer-efficiency` inverse: restores the previous `hammer_efficiency`.

use crate::change_pile_hammer_efficiency::ChangePileHammerEfficiency;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileHammerEfficiency, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_hammer_efficiency::ChangePileHammerEfficiency> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileHammerEfficiency { id: payload.id.clone(), new_hammer_efficiency: row.hammer_efficiency })],
        None => Vec::new(),
    })
}
