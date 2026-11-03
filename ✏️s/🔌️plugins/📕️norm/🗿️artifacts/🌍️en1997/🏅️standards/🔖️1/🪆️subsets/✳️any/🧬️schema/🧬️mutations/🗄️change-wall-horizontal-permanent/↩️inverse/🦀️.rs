//! ↩️ `change-wall-horizontal-permanent` inverse: restores the previous `horizontal_permanent`.

use crate::change_wall_horizontal_permanent::ChangeWallHorizontalPermanent;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeWallHorizontalPermanent, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_horizontal_permanent::ChangeWallHorizontalPermanent> {
    Ok(match base.retaining_walls.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWallHorizontalPermanent { id: payload.id.clone(), new_horizontal_permanent: row.horizontal_permanent })],
        None => Vec::new(),
    })
}
