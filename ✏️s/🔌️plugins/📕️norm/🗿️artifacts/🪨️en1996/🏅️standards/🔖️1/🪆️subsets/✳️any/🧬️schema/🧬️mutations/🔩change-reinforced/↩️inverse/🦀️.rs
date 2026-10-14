use crate::change_reinforced::ChangeReinforced;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeReinforced, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_reinforced::ChangeReinforced> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![Op::from(ChangeReinforced { index: payload.index, new_reinforced: base.walls[payload.index].reinforced })] }

    })())
}
