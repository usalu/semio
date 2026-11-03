//! ↩️ `change-floor-category` inverse: restores the previous `category`.

use crate::change_floor_category::ChangeFloorCategory;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeFloorCategory, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_floor_category::ChangeFloorCategory> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFloorCategory, base: &En1991Snapshot) -> Option<ChangeFloorCategory> {
    let r0 = base.floors.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFloorCategory { id: payload.id.clone(), new_category: r0.category.clone() })
}
