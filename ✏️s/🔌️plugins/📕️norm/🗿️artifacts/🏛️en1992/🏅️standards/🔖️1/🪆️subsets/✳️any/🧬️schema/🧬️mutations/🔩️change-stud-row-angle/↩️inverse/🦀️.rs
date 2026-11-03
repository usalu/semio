//! ↩️ `change-stud-row-angle` inverse: restores the previous `angle`.

use crate::change_stud_row_angle::ChangeStudRowAngle;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeStudRowAngle, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_stud_row_angle::ChangeStudRowAngle> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeStudRowAngle, base: &En1992Snapshot) -> Option<ChangeStudRowAngle> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let r1 = r0.stud_rows.iter().find(|row| row.id == payload.stud_row_id)?;
    Some(ChangeStudRowAngle { member_id: payload.member_id.clone(), stud_row_id: payload.stud_row_id.clone(), new_angle: r1.angle })
}
