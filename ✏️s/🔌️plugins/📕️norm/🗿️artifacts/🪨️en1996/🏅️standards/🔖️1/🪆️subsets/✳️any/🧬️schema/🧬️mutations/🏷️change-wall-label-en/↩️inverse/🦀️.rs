use crate::change_wall_label_en::ChangeWallLabelEn;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeWallLabelEn, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_label_en::ChangeWallLabelEn> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![Op::from(ChangeWallLabelEn { index: payload.index, new_label_en: base.walls[payload.index].label_en.clone() })] }

    })())
}
