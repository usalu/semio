//! ↩️ `change-cf-corner-radius` inverse: restores the previous `corner_radius`.

use crate::change_cf_corner_radius::ChangeCfCornerRadius;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCfCornerRadius, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cf_corner_radius::ChangeCfCornerRadius> {
    Ok(match base.cold_formed_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCfCornerRadius { id: payload.id.clone(), new_corner_radius: row.corner_radius })],
        None => Vec::new(),
    })
}
