//! ↩️ `change-transverse-sides` inverse: restores the previous `sides`.

use crate::change_transverse_sides::ChangeTransverseSides;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeTransverseSides, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_transverse_sides::ChangeTransverseSides> {
    Ok(match base.transverse_stiffeners.iter().find(|row| row.id == payload.transverse_stiffener_id) {
        Some(row) => vec![Op::from(ChangeTransverseSides { transverse_stiffener_id: payload.transverse_stiffener_id.clone(), new_sides: row.sides })],
        None => Vec::new(),
    })
}
