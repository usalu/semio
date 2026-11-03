//! ↩️ `change-buckling-length-column-stiffness-a` inverse: restores the previous `column_stiffness_a`.

use crate::change_buckling_length_column_stiffness_a::ChangeBucklingLengthColumnStiffnessA;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBucklingLengthColumnStiffnessA, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_buckling_length_column_stiffness_a::ChangeBucklingLengthColumnStiffnessA> {
    Ok(match base.buckling_lengths.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBucklingLengthColumnStiffnessA { id: payload.id.clone(), new_column_stiffness_a: row.column_stiffness_a })],
        None => Vec::new(),
    })
}
