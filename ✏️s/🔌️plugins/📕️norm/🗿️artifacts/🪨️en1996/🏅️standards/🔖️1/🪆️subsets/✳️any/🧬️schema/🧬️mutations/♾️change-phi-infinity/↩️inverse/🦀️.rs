//! ↩️ `change-phi-infinity` inverse: restores the previous `phi_infinity`.

use crate::change_phi_infinity::ChangePhiInfinity;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangePhiInfinity, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_phi_infinity::ChangePhiInfinity> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePhiInfinity, base: &En1996Snapshot) -> Option<ChangePhiInfinity> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangePhiInfinity { id: payload.id.clone(), new_phi_infinity: r0.phi_infinity })
}
