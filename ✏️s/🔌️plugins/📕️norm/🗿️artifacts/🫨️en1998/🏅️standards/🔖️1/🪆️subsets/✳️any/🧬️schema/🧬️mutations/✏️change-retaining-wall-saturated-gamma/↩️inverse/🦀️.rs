//! ↩️ Inverse for `change-retaining-wall-saturated-gamma`.
use crate::change_retaining_wall_saturated_gamma::ChangeRetainingWallSaturatedGamma;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeRetainingWallSaturatedGamma, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_retaining_wall_saturated_gamma::ChangeRetainingWallSaturatedGamma> {
    Ok(match base.retaining_walls.get(payload.index) {
        Some(row) => vec![Op::from(ChangeRetainingWallSaturatedGamma { index: payload.index, new_saturated_gamma: row.saturated_gamma })],
        None => Vec::new(),
    })
}
