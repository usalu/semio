//! ↩️ `change-buckling-length-sway` inverse: restores the previous `sway`.

use crate::change_buckling_length_sway::ChangeBucklingLengthSway;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBucklingLengthSway, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_buckling_length_sway::ChangeBucklingLengthSway> {
    Ok(match base.buckling_lengths.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBucklingLengthSway { id: payload.id.clone(), new_sway: row.sway })],
        None => Vec::new(),
    })
}
