//! ↩️ `change-member-second-order-analysis` inverse: restores the previous `second_order_analysis`.

use crate::change_member_second_order_analysis::ChangeMemberSecondOrderAnalysis;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberSecondOrderAnalysis, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_second_order_analysis::ChangeMemberSecondOrderAnalysis> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberSecondOrderAnalysis, base: &En1990Snapshot) -> Option<ChangeMemberSecondOrderAnalysis> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberSecondOrderAnalysis { id: payload.id.clone(), new_second_order_analysis: r0.second_order_analysis })
}
