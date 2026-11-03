//! ↩️ `change-buckling-length-restraint-a` inverse: restores the previous `restraint_a`.

use crate::change_buckling_length_restraint_a::ChangeBucklingLengthRestraintA;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBucklingLengthRestraintA, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_buckling_length_restraint_a::ChangeBucklingLengthRestraintA> {
    Ok(match base.buckling_lengths.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBucklingLengthRestraintA { id: payload.id.clone(), new_restraint_a: row.restraint_a })],
        None => Vec::new(),
    })
}
