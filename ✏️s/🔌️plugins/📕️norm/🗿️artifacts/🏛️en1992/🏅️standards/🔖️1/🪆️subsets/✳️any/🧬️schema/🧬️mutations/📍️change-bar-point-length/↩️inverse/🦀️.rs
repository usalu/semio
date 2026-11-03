//! ↩️ `change-bar-point-length` inverse: restores the previous `length`.

use crate::change_bar_point_length::ChangeBarPointLength;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeBarPointLength, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bar_point_length::ChangeBarPointLength> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBarPointLength, base: &En1992Snapshot) -> Option<ChangeBarPointLength> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let r1 = r0.bars.iter().find(|row| row.id == payload.bar_id)?;
    Some(ChangeBarPointLength { member_id: payload.member_id.clone(), bar_id: payload.bar_id.clone(), new_length: r1.length })
}
