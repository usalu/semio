//! ↩️ `change-bar-point-z` inverse: restores the previous `z`.

use crate::change_bar_point_z::ChangeBarPointZ;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeBarPointZ, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bar_point_z::ChangeBarPointZ> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBarPointZ, base: &En1992Snapshot) -> Option<ChangeBarPointZ> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let r1 = r0.bars.iter().find(|row| row.id == payload.bar_id)?;
    Some(ChangeBarPointZ { member_id: payload.member_id.clone(), bar_id: payload.bar_id.clone(), new_z: r1.z })
}
