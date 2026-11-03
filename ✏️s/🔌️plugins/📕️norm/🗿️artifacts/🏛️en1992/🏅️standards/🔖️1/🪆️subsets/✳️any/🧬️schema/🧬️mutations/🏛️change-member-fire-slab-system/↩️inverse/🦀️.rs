//! ↩️ `change-member-fire-slab-system` inverse: restores the previous `fire.slab_system`.

use crate::change_member_fire_slab_system::ChangeMemberFireSlabSystem;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberFireSlabSystem, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_fire_slab_system::ChangeMemberFireSlabSystem> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberFireSlabSystem, base: &En1992Snapshot) -> Option<ChangeMemberFireSlabSystem> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let object = r0.fire.as_ref()?;
    Some(ChangeMemberFireSlabSystem { member_id: payload.member_id.clone(), new_fire_slab_system: object.slab_system })
}
