use crate::change_unit_fb::ChangeUnitFb;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeUnitFb, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_unit_fb::ChangeUnitFb> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![Op::from(ChangeUnitFb { index: payload.index, new_f_b_pa: base.walls[payload.index].f_b_pa })] }

    })())
}
