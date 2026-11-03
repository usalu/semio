//! ↩️ `change-imposed-category` inverse: restores the previous `imposed_category`.

use crate::change_imposed_category::ChangeImposedCategory;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeImposedCategory, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_imposed_category::ChangeImposedCategory> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeImposedCategory, base: &En1996Snapshot) -> Option<ChangeImposedCategory> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let r1 = r0.load_cases.iter().find(|row| row.id == payload.id)?;
    Some(ChangeImposedCategory { wall_id: payload.wall_id.clone(), id: payload.id.clone(), new_imposed_category: r1.imposed_category })
}
