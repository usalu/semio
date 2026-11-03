//! ↩️ `change-member-prestress-steel-id` inverse: restores the previous `prestress_steel_id`.

use crate::change_member_prestress_steel_id::ChangeMemberPrestressSteelId;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberPrestressSteelId, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_prestress_steel_id::ChangeMemberPrestressSteelId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberPrestressSteelId, base: &En1992Snapshot) -> Option<ChangeMemberPrestressSteelId> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    Some(ChangeMemberPrestressSteelId { member_id: payload.member_id.clone(), new_prestress_steel_id: r0.prestress_steel_id.clone() })
}
