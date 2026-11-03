//! ↩️ `change-wall-label-en` inverse: restores the previous `label_en`.

use crate::change_wall_label_en::ChangeWallLabelEn;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeWallLabelEn, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_label_en::ChangeWallLabelEn> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeWallLabelEn, base: &En1996Snapshot) -> Option<ChangeWallLabelEn> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeWallLabelEn { id: payload.id.clone(), new_label_en: r0.label_en.clone() })
}
