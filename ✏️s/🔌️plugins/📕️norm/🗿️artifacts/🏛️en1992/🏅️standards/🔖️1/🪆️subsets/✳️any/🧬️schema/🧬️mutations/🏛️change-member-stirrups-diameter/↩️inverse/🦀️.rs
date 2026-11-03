//! ↩️ `change-member-stirrups-diameter` inverse: restores the previous `stirrups.diameter`.

use crate::change_member_stirrups_diameter::ChangeMemberStirrupsDiameter;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberStirrupsDiameter, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_stirrups_diameter::ChangeMemberStirrupsDiameter> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberStirrupsDiameter, base: &En1992Snapshot) -> Option<ChangeMemberStirrupsDiameter> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let object = r0.stirrups.as_ref()?;
    Some(ChangeMemberStirrupsDiameter { member_id: payload.member_id.clone(), new_stirrups_diameter: object.diameter })
}
