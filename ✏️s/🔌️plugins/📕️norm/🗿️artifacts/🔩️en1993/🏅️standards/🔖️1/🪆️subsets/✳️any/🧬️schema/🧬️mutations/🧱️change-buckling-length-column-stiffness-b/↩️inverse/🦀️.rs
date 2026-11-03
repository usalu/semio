//! ↩️ `change-buckling-length-column-stiffness-b` inverse: restores the previous `column_stiffness_b`.

use crate::change_buckling_length_column_stiffness_b::ChangeBucklingLengthColumnStiffnessB;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBucklingLengthColumnStiffnessB, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_buckling_length_column_stiffness_b::ChangeBucklingLengthColumnStiffnessB> {
    Ok(match base.buckling_lengths.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBucklingLengthColumnStiffnessB { id: payload.id.clone(), new_column_stiffness_b: row.column_stiffness_b })],
        None => Vec::new(),
    })
}
