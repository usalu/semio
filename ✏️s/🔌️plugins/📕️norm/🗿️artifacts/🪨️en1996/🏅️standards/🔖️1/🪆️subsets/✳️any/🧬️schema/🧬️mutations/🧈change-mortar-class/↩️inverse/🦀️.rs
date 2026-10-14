use crate::change_mortar_class::ChangeMortarClass;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeMortarClass, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_mortar_class::ChangeMortarClass> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![Op::from(ChangeMortarClass { index: payload.index, new_mortar_class: base.walls[payload.index].mortar_class })] }

    })())
}
