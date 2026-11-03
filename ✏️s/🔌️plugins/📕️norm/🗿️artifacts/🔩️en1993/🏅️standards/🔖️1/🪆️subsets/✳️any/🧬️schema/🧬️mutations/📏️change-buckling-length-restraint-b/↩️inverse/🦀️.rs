//! ↩️ `change-buckling-length-restraint-b` inverse: restores the previous `restraint_b`.

use crate::change_buckling_length_restraint_b::ChangeBucklingLengthRestraintB;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBucklingLengthRestraintB, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_buckling_length_restraint_b::ChangeBucklingLengthRestraintB> {
    Ok(match base.buckling_lengths.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBucklingLengthRestraintB { id: payload.id.clone(), new_restraint_b: row.restraint_b })],
        None => Vec::new(),
    })
}
