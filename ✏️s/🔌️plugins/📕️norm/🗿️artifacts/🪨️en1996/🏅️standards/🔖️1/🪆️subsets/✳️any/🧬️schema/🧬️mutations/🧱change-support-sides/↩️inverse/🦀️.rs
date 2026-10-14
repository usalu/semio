use crate::change_support_sides::ChangeSupportSides;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeSupportSides, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_support_sides::ChangeSupportSides> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![Op::from(ChangeSupportSides { index: payload.index, new_support_sides: base.walls[payload.index].support_sides })] }

    })())
}
