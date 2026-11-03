//! ↩️ `change-bridge-pier-cracked-inertia-ratio` inverse: restores the previous `pier_cracked_inertia_ratio`.

use crate::change_bridge_pier_cracked_inertia_ratio::ChangeBridgePierCrackedInertiaRatio;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgePierCrackedInertiaRatio, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_pier_cracked_inertia_ratio::ChangeBridgePierCrackedInertiaRatio> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBridgePierCrackedInertiaRatio, base: &En1998Snapshot) -> Option<ChangeBridgePierCrackedInertiaRatio> {
    let r0 = base.bridges.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBridgePierCrackedInertiaRatio { id: payload.id.clone(), new_pier_cracked_inertia_ratio: r0.pier_cracked_inertia_ratio })
}
