//! ↩️ `change-member-rd-geo` inverse: restores the previous `rd_geo`.

use crate::change_member_rd_geo::ChangeMemberRdGeo;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberRdGeo, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_rd_geo::ChangeMemberRdGeo> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberRdGeo, base: &En1990Snapshot) -> Option<ChangeMemberRdGeo> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberRdGeo { id: payload.id.clone(), new_rd_geo: r0.rd_geo })
}
