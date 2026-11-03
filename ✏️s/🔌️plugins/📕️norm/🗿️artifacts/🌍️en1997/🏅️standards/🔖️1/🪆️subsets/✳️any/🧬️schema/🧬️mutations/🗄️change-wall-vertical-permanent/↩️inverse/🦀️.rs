//! ↩️ `change-wall-vertical-permanent` inverse: restores the previous `vertical_permanent`.

use crate::change_wall_vertical_permanent::ChangeWallVerticalPermanent;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeWallVerticalPermanent, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_vertical_permanent::ChangeWallVerticalPermanent> {
    Ok(match base.retaining_walls.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWallVerticalPermanent { id: payload.id.clone(), new_vertical_permanent: row.vertical_permanent })],
        None => Vec::new(),
    })
}
