//! ↩️ `change-pile-bending-stiffness-ei` inverse: restores the previous `bending_stiffness_ei`.

use crate::change_pile_bending_stiffness_ei::ChangePileBendingStiffnessEi;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileBendingStiffnessEi, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_bending_stiffness_ei::ChangePileBendingStiffnessEi> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileBendingStiffnessEi { id: payload.id.clone(), new_bending_stiffness_ei: row.bending_stiffness_ei })],
        None => Vec::new(),
    })
}
