//! ↩️ `change-member-prestress-loss-ratio` inverse: restores the previous `prestress.loss_ratio`.

use crate::change_member_prestress_loss_ratio::ChangeMemberPrestressLossRatio;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberPrestressLossRatio, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_prestress_loss_ratio::ChangeMemberPrestressLossRatio> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberPrestressLossRatio, base: &En1992Snapshot) -> Option<ChangeMemberPrestressLossRatio> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let object = r0.prestress.as_ref()?;
    Some(ChangeMemberPrestressLossRatio { member_id: payload.member_id.clone(), new_prestress_loss_ratio: object.loss_ratio })
}
