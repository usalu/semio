use crate::change_f_yd::ChangeFYd;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeFYd, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_f_yd::ChangeFYd> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![Op::from(ChangeFYd { index: payload.index, new_f_yd_pa: base.walls[payload.index].f_yd_pa })] }

    })())
}
