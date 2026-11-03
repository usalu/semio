//! ↩️ `change-foundation-pile-iface-m` inverse: restores the previous `pile_interface_depth_m`.

use crate::change_foundation_pile_iface_m::ChangeFoundationPileIfaceM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationPileIfaceM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_pile_iface_m::ChangeFoundationPileIfaceM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFoundationPileIfaceM, base: &En1998Snapshot) -> Option<ChangeFoundationPileIfaceM> {
    let r0 = base.foundations.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFoundationPileIfaceM { id: payload.id.clone(), new_pile_interface_depth_m: r0.pile_interface_depth_m })
}
