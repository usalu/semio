use crate::change_mortar_type::ChangeMortarType;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeMortarType, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_mortar_type::ChangeMortarType> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![Op::from(ChangeMortarType { index: payload.index, new_mortar_type: base.walls[payload.index].mortar_type })] }

    })())
}
