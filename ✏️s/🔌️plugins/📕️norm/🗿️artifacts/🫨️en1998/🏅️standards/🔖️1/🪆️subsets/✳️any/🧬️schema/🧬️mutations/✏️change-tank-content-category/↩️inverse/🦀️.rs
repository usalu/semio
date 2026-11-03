//! ↩️ `change-tank-content-category` inverse: restores the previous `content_category`.

use crate::change_tank_content_category::ChangeTankContentCategory;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTankContentCategory, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tank_content_category::ChangeTankContentCategory> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTankContentCategory, base: &En1998Snapshot) -> Option<ChangeTankContentCategory> {
    let r0 = base.tanks.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTankContentCategory { id: payload.id.clone(), new_content_category: r0.content_category })
}
