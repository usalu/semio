use crate::change_phi_infinity::ChangePhiInfinity;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangePhiInfinity, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_phi_infinity::ChangePhiInfinity> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else {
        vec![Op::from(ChangePhiInfinity { index: payload.index, new_phi_infinity: base.walls[payload.index].phi_infinity })]
    }

    })())
}
