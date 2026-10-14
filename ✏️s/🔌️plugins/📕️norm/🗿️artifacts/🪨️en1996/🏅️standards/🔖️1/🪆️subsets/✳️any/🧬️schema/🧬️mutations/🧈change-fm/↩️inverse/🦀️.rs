use crate::change_fm::ChangeFm;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeFm, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fm::ChangeFm> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![Op::from(ChangeFm { index: payload.index, new_mortar_strength_pa: base.walls[payload.index].mortar_strength_pa })] }

    })())
}
