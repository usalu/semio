//! ↩️ `change-wall-label-de` inverse: restores the previous `label_de`.

use crate::change_wall_label_de::ChangeWallLabelDe;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeWallLabelDe, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_label_de::ChangeWallLabelDe> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeWallLabelDe, base: &En1996Snapshot) -> Option<ChangeWallLabelDe> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeWallLabelDe { id: payload.id.clone(), new_label_de: r0.label_de.clone() })
}
