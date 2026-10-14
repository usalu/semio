use crate::change_wall_label_de::ChangeWallLabelDe;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeWallLabelDe, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_label_de::ChangeWallLabelDe> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![Op::from(ChangeWallLabelDe { index: payload.index, new_label_de: base.walls[payload.index].label_de.clone() })] }

    })())
}
